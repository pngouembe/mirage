pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use mirage::{sync, Config, Link, MIRAGE_BACKUP_FILE_EXTENSION};
use std::{env, fs};
use tempdir::TempDir;

#[test]
fn test_replicate_folder_in_folder() -> Result<()> {
    let tmp_dir = TempDir::new("test_replicate_folder_in_folder")?;

    let folder_to_replicate = env::current_dir()?.join("tests/resources/folder_to_replicate");

    mirage::create_links(&folder_to_replicate, tmp_dir.path(), true)?;

    for (src_entry, dst_entry) in
        fs::read_dir(&folder_to_replicate)?.zip(fs::read_dir(tmp_dir.path())?)
    {
        let src_entry = src_entry?;
        let dst_entry = dst_entry?;

        assert!(dst_entry.metadata()?.file_type().is_symlink());

        assert_eq!(fs::read_link(dst_entry.path())?, src_entry.path());
    }

    Ok(())
}

#[test]
fn test_replicate_file_in_folder() -> Result<()> {
    let tmp_dir = TempDir::new("test_replicate_file_in_folder")?;

    let file_to_replicate = env::current_dir()?.join("tests/resources/folder_to_replicate/file_a");

    let dst_file = tmp_dir.path().join("file_a");

    mirage::create_links(&file_to_replicate, tmp_dir.path(), true)?;

    assert!(dst_file.exists());

    assert_eq!(fs::read_link(dst_file)?, file_to_replicate);

    let unwanted_source_file_backup =
        file_to_replicate.with_extension(MIRAGE_BACKUP_FILE_EXTENSION);
    assert!(!unwanted_source_file_backup.exists());

    Ok(())
}

#[test]
fn test_replicate_file_in_folder_with_backup() -> Result<()> {
    let tmp_dir = TempDir::new("test_replicate_file_in_folder_with_backup")?;

    let file_to_replicate = env::current_dir()?.join("tests/resources/folder_to_replicate/file_a");

    let dst_file = tmp_dir.path().join("file_a");

    mirage::create_links(&file_to_replicate, tmp_dir.path(), false)?;
    mirage::create_links(&file_to_replicate, tmp_dir.path(), true)?;

    assert!(dst_file.exists());

    assert_eq!(fs::read_link(&dst_file)?, file_to_replicate);

    let unwanted_source_file_backup =
        file_to_replicate.with_extension(MIRAGE_BACKUP_FILE_EXTENSION);
    assert!(!unwanted_source_file_backup.exists());

    let wanted_dst_file_backup = dst_file.with_extension(MIRAGE_BACKUP_FILE_EXTENSION);
    assert!(wanted_dst_file_backup.exists());

    Ok(())
}

#[test]
fn test_replicate_file_with_new_name() -> Result<()> {
    let tmp_dir = TempDir::new("test_replicate_file_with_new_name")?;
    let file_to_replicate = env::current_dir()?.join("tests/resources/folder_to_replicate/file_a");
    let dst_file = tmp_dir.path().join("replicated_file_a");

    mirage::create_links(&file_to_replicate, &dst_file, true)?;

    assert!(dst_file.exists());

    assert_eq!(fs::read_link(dst_file)?, file_to_replicate);

    let unwanted_source_file_backup =
        file_to_replicate.with_extension(MIRAGE_BACKUP_FILE_EXTENSION);
    assert!(!unwanted_source_file_backup.exists());

    Ok(())
}

#[test]
fn test_sync_folder() -> Result<()> {
    let tmp_dir = TempDir::new("test_sync_folder()")?;
    let folder_to_replicate = env::current_dir()?.join("tests/resources/folder_to_replicate/");
    let config = Config {
        links_to_do: vec![Link {
            source: folder_to_replicate.clone(),
            destination: tmp_dir.path().to_owned(),
        }],
    };

    sync(config)?;

    for (src_entry, dst_entry) in
        fs::read_dir(&folder_to_replicate)?.zip(fs::read_dir(tmp_dir.path())?)
    {
        let src_entry = src_entry?;
        let dst_entry = dst_entry?;

        assert!(dst_entry.metadata()?.file_type().is_symlink());

        assert_eq!(fs::read_link(dst_entry.path())?, src_entry.path());
    }

    Ok(())
}
