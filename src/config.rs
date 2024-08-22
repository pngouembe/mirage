use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub source: PathBuf,
    pub destination: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub links_to_do: Vec<Link>,
}

impl Config {
    pub fn try_from_file(file_path: &Path) -> Result<Config> {
        if !file_path.exists() {
            return Err(format!(
                "Config file not found, {} doesn't exist",
                file_path.display()
            )
            .into());
        }

        let config = serde_yaml::from_str(&fs::read_to_string(file_path)?)?;

        Ok(config)
    }
}
