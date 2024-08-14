use std::path::{Path, PathBuf};

use futures;
use tokio::fs;
use tokio_stream::StreamExt;

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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let source_path = fs::canonicalize(&args.source_path).await?;
    let destination_path = fs::canonicalize(&args.destination_path).await?;

    check_source_path(&source_path)?;
    check_destination_path(&destination_path)?;

    let items_to_replicate = get_source_path_items(&source_path).await?;

    // TODO: avoid the cloning

    let handles = items_to_replicate
        .into_iter()
        .fold(Vec::new(), |mut handles, item| {
            let source_path = source_path.clone();
            let destination_path = destination_path.clone();

            handles.push(tokio::spawn(async move {
                replicate_item(&item.clone(), &source_path, &destination_path)
                    .await
                    .unwrap();
            }));
            handles
        });

    futures::future::join_all(handles).await;

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

async fn get_source_path_items(source_path: &PathBuf) -> Result<Vec<PathBuf>> {
    // TODO: avoid the cloning
    if !source_path.is_dir() {
        return Ok(vec![source_path.clone()]);
    }

    let items = fs::read_dir(source_path).await?;

    let item_stream = tokio_stream::wrappers::ReadDirStream::new(items);

    Ok(item_stream
        .filter_map(|entry| {
            if let Ok(e) = entry {
                Some(e.path())
            } else {
                None
            }
        })
        .collect()
        .await)
}

async fn replicate_item(item: &Path, source_path: &Path, destination_path: &Path) -> Result<()> {
    let item_relative_path = item.strip_prefix(source_path)?;
    let item_destination_path = destination_path.join(item_relative_path);

    create_symlink(item, &item_destination_path).await?;

    Ok(())
}

async fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if cfg!(windows) {
        return Err("Windows is not supported yet".into());
    }

    fs::symlink(source, destination).await?;

    Ok(())
}
