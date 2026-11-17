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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::loader::{FileConfig, IgnoreConfig};

    #[test]
    fn test_validate_rejects_absolute_ignore_paths() {
        let config = FileConfig {
            ignore: IgnoreConfig {
                paths: vec!["/etc/passwd".to_string()],
            },
            ..Default::default()
        };

        let result = validate_config(&config);
        assert!(result.is_err());

        let err_message = result.unwrap_err().to_string();
        assert!(err_message.contains("must be relative"));
    }

    #[test]
    fn test_validate_relative_ignore_paths() {
        let config = FileConfig {
            ignore: IgnoreConfig {
                paths: vec!["legacy".to_string(), "tmp/build".to_string()],
            },
            ..Default::default()
        };

        let result = validate_config(&config);
        assert!(result.is_ok());
    }
}
