use std::collections::HashMap;

use crate::{
    config::loader::{FileConfig, IgnoreConfig},
    models::Severity,
};

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            ignore: IgnoreConfig { paths: Vec::new() },
            rules: HashMap::new(),
            default_severity: Severity::Warning,
        }
    }
}

pub fn get_default_config() -> FileConfig {
    FileConfig::default()
}
