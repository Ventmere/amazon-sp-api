use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, path::{Path, PathBuf}, process};

pub fn run(filter_module: Option<&str>) -> Result<()> {
  let models_dir = fs::read_dir("generated")?;

  for entry in models_dir {
    let path = entry?.path();

    tracing::info!("publishing {}...", path.display());

    let output = process::Command::new("cargo")
      .arg("publish")
      .arg("--manifest-path")
      .arg(path.join("Cargo.toml"))
      .output()?;

    if output.status.success() {
      tracing::info!("published {}", path.display());
    } else {
      tracing::error!("failed to publish {}:\n {}", path.display(), String::from_utf8_lossy(&output.stderr));
      return Err(anyhow::anyhow!("failed to publish"));
    }
  }

  Ok(())
}