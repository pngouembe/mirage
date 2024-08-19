use std::{path::PathBuf, time::Instant};

use mirage::create_links;

use clap::{Parser, Subcommand};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

// TODO: Add commands (install, clean)
// TODO: Add an override option to override existing files (backup or not)

/// Program used to replicate a directory tree in an other location using symlinks
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Install {
        /// Path to the folder containing the files and folders to replicate
        #[clap(short, long = "src")]
        source_path: PathBuf,

        /// Path to the folder were the files and folders will be replicated
        #[clap(short, long = "dst")]
        destination_path: PathBuf,

        /// Override existing files
        #[clap(short, long)]
        no_backup: bool,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let start = Instant::now();
    let args = Args::parse();

    match args.command {
        Command::Install {
            source_path,
            destination_path,
            no_backup,
        } => create_links(&source_path, &destination_path, !no_backup)?,
    }

    let elapsed_time = start.elapsed();

    log::info!("Took {:?}", elapsed_time);

    Ok(())
}
