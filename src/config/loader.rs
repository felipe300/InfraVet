use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};

use crate::{
    config::{defaults::get_default_config, validation::validate_config},
    models::Severity,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FileConfig {
    #[serde(default)]
    pub ignore: IgnoreConfig,

    #[serde(default)]
    pub rules: HashMap<String, RuleConfigurationOverride>,

    #[serde(default)]
    pub default_severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IgnoreConfig {
    #[serde(default)]
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleConfigurationOverride {
    Severity(Severity),
    Detailed {
        #[serde(default = "default_true")]
        enabled: bool,
        #[serde(default)]
        severity: Option<Severity>,
    },
}

const fn default_true() -> bool {
    true
}

impl FileConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();

        let content = read_to_string(path_ref).with_context(|| {
            format!("Failed to read configuration file: {}", path_ref.display())
        })?;
        let config: Self = toml::from_str(&content).with_context(|| {
            format!("Failed to parse TOML configuration: {}", path_ref.display())
        })?;

        validate_config(&config)?;

        Ok(config)
    }

    pub fn load_for_scan(target_root: &Path, explicit_path: Option<&Path>) -> Result<Self> {
        let config_path = match explicit_path {
            Some(path) => {
                println!("Loading explicit configuration: {}", path.display());
                return Self::load_from_file(path);
            }
            None => target_root.join("infravet.toml"),
        };

        if config_path.try_exists()? {
            println!("Loading configuration: {}", config_path.display());
            Self::load_from_file(config_path)
        } else {
            println!("No infravet.toml found. Using default configuration.");
            Ok(get_default_config())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Severity;
    use tempfile::tempdir;

    #[test]
    fn test_load_custom_infravet_toml() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("infravet.toml");

        let toml_content = r#"
            default-severity = "WARNING"

            [ignore]
            paths = ["custom_ignore/"]

            [rules]
            "DF001" = "ERROR"
            "DF002" = { enabled = false }
        "#;

        std::fs::write(&config_path, toml_content).unwrap();

        let config = FileConfig::load_from_file(&config_path).unwrap();

        assert_eq!(config.default_severity, Severity::Warning);
        assert_eq!(config.ignore.paths, vec!["custom_ignore/".to_string()]);
        assert!(config.rules.contains_key("DF001"));
        assert!(config.rules.contains_key("DF002"));
    }

    #[test]
    fn test_load_default_configuration_fallback() {
        let dir = tempdir().unwrap();
        // Intentamos cargar la configuración para un directorio donde no existe infravet.toml
        let config = FileConfig::load_for_scan(dir.path(), None).unwrap();

        // Debe cargar los valores por defecto definidos en defaults.rs
        assert!(config.rules.is_empty());
        assert!(config.ignore.paths.is_empty());
    }
}
