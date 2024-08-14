use std::{
    fs, os,
    path::{Path, PathBuf},
};

use clap::Parser;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

// TODO: Add commands (install, clean)
// TODO: Add an override option to override existing files (backup or not)

/// Program used to replicate a directory tree in an other location using symlinks
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the folder containing the files and folders to replicate
    #[arg(short, long = "src")]
    source_path: PathBuf,

    /// Path to the folder were the files and folders will be replicated
    #[arg(short, long = "dst")]
    destination_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source_path = fs::canonicalize(&args.source_path)?;
    let destination_path = fs::canonicalize(&args.destination_path)?;

    check_source_path(&source_path)?;
    check_destination_path(&destination_path)?;

    let items_to_replicate = get_source_path_items(&source_path)?;

    items_to_replicate
        .iter()
        .try_for_each(|item| replicate_item(item, &source_path, &destination_path))?;

    Ok(())
}

fn check_source_path(source_path: &Path) -> Result<()> {
    if !source_path.exists() {
        return Err(format!("{} does not exist", source_path.display()).into());
    }

    if !source_path.is_dir() {
        return Err(format!("source path is not a directory ({})", source_path.display()).into());
    }

    Ok(())
}

fn check_destination_path(destination_path: &Path) -> Result<()> {
    if !destination_path.exists() {
        return Err(format!("{} does not exist", destination_path.display()).into());
    }

    if !destination_path.is_dir() {
        return Err(format!(
            "destination path is not a directory ({})",
            destination_path.display()
        )
        .into());
    }

    Ok(())
}

fn get_source_path_items(source_path: &PathBuf) -> Result<Vec<PathBuf>> {
    if !source_path.is_dir() {
        return Ok(vec![source_path.clone()]);
    }

    Ok(fs::read_dir(source_path)?
        .filter_map(|entry| {
            if let Ok(e) = entry {
                Some(e.path())
            } else {
                None
            }
        })
        .collect())
}

fn replicate_item(item: &Path, source_path: &Path, destination_path: &Path) -> Result<()> {
    let item_relative_path = item.strip_prefix(source_path)?;
    let item_destination_path = destination_path.join(item_relative_path);

    create_symlink(item, &item_destination_path)?;

    Ok(())
}

fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if cfg!(windows) {
        return Err("Windows is not supported yet".into());
    }

    os::unix::fs::symlink(source, destination)?;

    Ok(())
}
