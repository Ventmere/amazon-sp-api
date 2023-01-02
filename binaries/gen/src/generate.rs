use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, path::{Path, PathBuf}, process, io::Write};

pub fn run(filter_module: Option<&str>) -> Result<()> {
  let models_dir = fs::read_dir("deps/selling-partner-api-models/models")?;

  for entry in models_dir {
    let path = entry?.path();
    let module = parse_module(path)?;
    if let Some(name) = filter_module {
      if module.name != name {
        continue;
      }
    }
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
    const CRATE_OUT_DIR: &str = "generated";

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
      .arg(&out_path)
      .arg("-c")
      .arg("openapi/config.yaml")
      .output()?;
    if !output.status.success() {
      tracing::error!("openapi-generator exited with error:");
      std::io::stderr().write_all(&output.stderr)?;
      return Err(anyhow::format_err!("openapi-generator execution failed"));
    }

    let lib_rs_content = fs::read(out_path.join("src/lib.rs"))?;
    let updated: Vec<_> = "#![allow(nonstandard_style)]\n".as_bytes().iter()
      .cloned()
      .chain(lib_rs_content.into_iter())
      .collect();
    fs::write(out_path.join("src/lib.rs"), updated)?;

    if self.name == "orders" {
      // https://github.com/amzn/selling-partner-api-docs/issues/480

      let replaces = &[
        ("src/models/product_info_detail.rs", r##"#[serde(default, rename = "NumberOfItems", skip_serializing_if = "Option::is_none")]"##, r##"#[serde(default, rename = "NumberOfItems", skip_serializing_if = "Option::is_none", deserialize_with = "amazon_sp_api_shared::helpers::deserialize_opt_i32_with_parse")]"##),
        ("src/models/order_item.rs", r##"#[serde(default, rename = "IsGift", skip_serializing_if = "Option::is_none")]"##, r##"#[serde(default, rename = "IsGift", skip_serializing_if = "Option::is_none", deserialize_with = "amazon_sp_api_shared::helpers::deserialize_opt_bool_from_string")]"##),
        ("src/models/buyer_requested_cancel.rs", r##"#[serde(default, rename = "IsBuyerRequestedCancel", skip_serializing_if = "Option::is_none")]"##, r##"#[serde(default, rename = "IsBuyerRequestedCancel", skip_serializing_if = "Option::is_none", deserialize_with = "amazon_sp_api_shared::helpers::deserialize_opt_bool_from_string")]"##),
      ];

      for (file, from, to) in replaces {
        let lib_rs_content = fs::read_to_string(out_path.join(file))?;
        let updated = lib_rs_content.replace(
          from, 
          to
        );
        fs::write(out_path.join(file), updated)?;
      }      
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