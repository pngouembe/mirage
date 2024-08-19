pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use std::{env, fs};
use tempdir::TempDir;

#[test]
fn test_replicate_folder() -> Result<()> {
    let tmp_dir = TempDir::new("test_replicate_folder")?;

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
fn test_replicate_file() -> Result<()> {
    let tmp_dir = TempDir::new("test_replicate_file")?;

    let file_to_replicate = env::current_dir()?.join("tests/resources/folder_to_replicate/file_a");

    let dst_file = tmp_dir.path().join("file_a");

    mirage::create_links(&file_to_replicate, tmp_dir.path(), true)?;

    dbg!(&dst_file);

    assert!(dst_file.exists());

    assert_eq!(fs::read_link(dst_file)?, file_to_replicate);

    Ok(())
}
