use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, path::{Path, PathBuf}, process};

pub fn run() -> Result<()> {
  let models_dir = fs::read_dir("deps/selling-partner-api-models/models")?;

  for entry in models_dir {
    let path = entry?.path();
    let module = parse_module(path)?;
    module.generate()?;
  }

  Ok(())
}

#[derive(Debug)]
struct ModuleInfo {
  name: String,
  path: PathBuf,
  prefix: String,
  versions: Vec<Version>,
}

impl ModuleInfo {
  fn get_filename(&self) -> String {
    let v = match self.versions.last().unwrap() {
      Version::NoVersion => format!(""),
      Version::Number(v) => format!("V{}", v),
      Version::Date(v) => format!("_{}", v),
    };
    format!("{}{}.json", self.prefix, v)
  }

  fn generate(&self) -> Result<()> {
    const CRATE_OUT_DIR: &str = "crates";

    tracing::info!("generating {}...", self.name);

    let package = format!("amazon-sp-{}", self.name);
    let in_path = self.path.join(self.get_filename());
    let out_path = Path::new(CRATE_OUT_DIR).join(&package);
    let output = process::Command::new("openapi-generator")
      .arg("generate")
      .arg("--skip-validate-spec")
      .arg("-p")
      .arg(format!("packageName={}", package))
      .arg("-p")
      .arg("packageVersion=0.1.0")
      .arg("-g")
      .arg("rust")
      .arg("-i")
      .arg(in_path)
      .arg("-o")
      .arg(out_path)
      .output()?;
    if !output.status.success() {
      tracing::error!("{:?}", output);
      return Err(anyhow::format_err!("openapi-generator execution failed"));
    }
    Ok(())
  }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Version {
  NoVersion,
  Number(i32),
  Date(String),
}

fn parse_module(path: PathBuf) -> Result<ModuleInfo> {
  let dir = fs::read_dir(&path)?;
  let names: Vec<String> = dir
    .into_iter()
    .filter_map(|entry| {
      let entry = entry.ok()?;
      let filename = entry.file_name();
      let name = filename.to_str()?;
      if name.starts_with('.') {
        return None;
      }
      Some(name.to_string())
    })
    .collect();

  let mut prefix = None;
  let mut versions = Vec::with_capacity(names.len());
  if names.len() == 1 {
    let (filename, version) = split_filename(&names[0])
      .ok_or_else(|| anyhow::format_err!("cannot get prefix: {:?}", path))?;
    prefix.replace(filename.to_string());
    versions.push(version);
  } else {
    for name in &names {
      let (filename, version) = split_filename(name)
        .ok_or_else(|| anyhow::format_err!("cannot get prefix: {:?}", path))?;

      if let Some(ref v) = prefix {
        if filename != v {
          return Err(anyhow::format_err!("prefix mismatch: {:?}", path));
        }
      } else {
        prefix.replace(filename.to_string());
      }
      versions.push(version);
    }
  }

  let prefix = prefix.ok_or_else(|| anyhow::format_err!("cannot get prefix: {:?}", path))?;

  let name = path
    .file_name()
    .and_then(|v| v.to_str())
    .ok_or_else(|| anyhow::format_err!("cannot get filename: {:?}", path))?;

  let suffixes = &["-api-model", "-model"];
  let mut found_suffix = None;
  for suffix in suffixes {
    if name.ends_with(suffix) {
      found_suffix.replace(suffix);
      break;
    }
  }
  let name = found_suffix
    .map(|suffix| &name[0..(name.len() - suffix.len())])
    .ok_or_else(|| anyhow::format_err!("unexpected filename: {:?}", path))?
    .to_string();

  versions.sort();
  versions.reverse();

  Ok(ModuleInfo {
    path,
    name,
    prefix,
    versions,
  })
}

fn split_filename(filename: &str) -> Option<(&str, Version)> {
  static NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"V(\d+)\.json$").unwrap());
  static DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"_(\d{4}-\d{2}-\d{2}).json$").unwrap());

  let filename_len = filename.len();
  if let Some(caps) = NUMBER.captures(filename) {
    let n_str = caps.get(1).unwrap().as_str();
    let n = n_str.parse::<i32>().ok()?;
    return Some((
      &filename[0..(filename_len - "V0.json".len() + (n_str.len() - 1))],
      Version::Number(n),
    ));
  }

  if let Some(caps) = DATE.captures(filename) {
    let suffix_len = "_YYYY-MM-DD.json".len();
    return Some((
      &filename[0..(filename_len - suffix_len)],
      Version::Date(caps.get(1).unwrap().as_str().to_string()),
    ));
  }

  if filename.ends_with(".json") {
    return Some((
      &filename[0..(filename_len - ".json".len())],
      Version::NoVersion,
    ));
  }

  None
}