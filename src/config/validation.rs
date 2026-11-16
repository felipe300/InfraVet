use anyhow::{Result, bail};

use crate::config::loader::FileConfig;

pub fn validate_config(config: &FileConfig) -> Result<()> {
    for path in &config.ignore.paths {
        if path.starts_with('/') || path.starts_with('\\') {
            bail!(
                "Invalid ignore path '{path}': Ignore paths must be relative to the project root, not absolute."
            );
        }
    }

    for rule_id in config.rules.keys() {
        if rule_id.trim().is_empty() {
            bail!("Configuration error: Found an empty rule identifier override.");
        }
    }

    Ok(())
}
