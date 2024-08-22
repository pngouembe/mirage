use crate::{create_links, Result};

use crate::config::Config;

pub fn sync(config: Config) -> Result<()> {
    for link in config.links_to_do {
        create_links(&link.source, &link.destination, true)?;
    }

    Ok(())
}
