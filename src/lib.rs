use std::{
    fs, os,
    path::{Path, PathBuf},
};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

const MIRAGE_BACKUP_FILE_EXTENSION: &str = "mirage.backup";

pub fn create_links(source_path: &Path, destination_path: &Path, backup: bool) -> Result<()> {
    let source_path = fs::canonicalize(source_path)?;
    let destination_path = fs::canonicalize(destination_path)?;

    log::debug!(
        "Start replication: {} -> {}",
        source_path.display(),
        destination_path.display()
    );

    check_source_path(&source_path)?;
    check_destination_path(&destination_path)?;

    match (source_path.is_dir(), destination_path.is_dir()) {
        (false, true) => replicate_file_in_folder(&source_path, &destination_path, backup),
        (true, true) => replicate_folder(&source_path, &destination_path, backup),
        (_, _) => Err("The source path and/or destination path are not compatible.".into()),
    }
}

fn check_source_path(source_path: &Path) -> Result<()> {
    if !source_path.exists() {
        return Err(format!("{} does not exist", source_path.display()).into());
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

fn replicate_file_in_folder(
    source_path: &Path,
    destination_path: &Path,
    backup: bool,
) -> Result<()> {
    let Some(file_name) = source_path.file_name() else {
        return Err("Couldn't find the name of the source file".into());
    };

    let destination_path = destination_path.join(file_name);

    // TODO: extract in a function
    if destination_path.exists() {
        if backup {
            backup_item(&destination_path)?;
        } else {
            remove_item(&destination_path)?;
        }
    }

    create_symlink(source_path, &destination_path)?;

    Ok(())
}

fn replicate_folder(source_path: &Path, destination_path: &Path, backup: bool) -> Result<()> {
    let items_to_replicate = get_source_path_items(source_path)?;

    items_to_replicate
        .iter()
        .try_for_each(|item| replicate_item(item, source_path, destination_path, backup))?;

    Ok(())
}

fn get_source_path_items(source_path: &Path) -> Result<Vec<PathBuf>> {
    if !source_path.is_dir() {
        return Ok(vec![source_path.to_owned()]);
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

fn replicate_item(
    item: &Path,
    source_path: &Path,
    destination_path: &Path,
    backup: bool,
) -> Result<()> {
    let item_relative_path = item.strip_prefix(source_path)?;
    let item_destination_path = destination_path.join(item_relative_path);

    if item_destination_path.exists() {
        if backup {
            backup_item(&item_destination_path)?;
        } else {
            remove_item(&item_destination_path)?;
        }
    }

    create_symlink(item, &item_destination_path)?;

    Ok(())
}

fn backup_item(destination: &Path) -> Result<()> {
    let backup_item_extension = match destination.extension() {
        None => MIRAGE_BACKUP_FILE_EXTENSION.to_string(),
        Some(ext) => format!("{}.{}", ext.to_string_lossy(), MIRAGE_BACKUP_FILE_EXTENSION),
    };

    let backup_item = destination.with_extension(backup_item_extension);

    log::debug!(
        "Backing up {} (renamed {})",
        destination.display(),
        backup_item.display()
    );

    fs::rename(destination, backup_item)?;

    Ok(())
}

fn remove_item(destination: &Path) -> Result<()> {
    log::debug!("Removing {}", destination.display());
    if destination.exists() {
        if destination.is_dir() {
            fs::remove_dir_all(destination)?;
        } else {
            fs::remove_file(destination)?;
        }
    }
    Ok(())
}

fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if cfg!(windows) {
        return Err("Windows is not supported yet".into());
    }

    log::debug!(
        "Creating link: {} -> {}",
        source.display(),
        destination.display()
    );

    os::unix::fs::symlink(source, destination)?;

    Ok(())
}
