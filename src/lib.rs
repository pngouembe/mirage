mod config;
mod error;
mod linker;
mod synchronizer;

pub use error::{Error, Result};

pub use config::{Config, Link};
pub use linker::{create_links, MIRAGE_BACKUP_FILE_EXTENSION};
pub use synchronizer::sync;
