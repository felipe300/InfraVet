use clap::ValueEnum;
use serde::Deserialize;

#[derive(Debug, Clone, ValueEnum, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum FileType {
    #[value(name = "dockerfile")]
    Dockerfile,

    #[value(name = "compose")]
    Compose,

    #[value(name = "terraform")]
    Terraform,

    #[value(name = "kubernetes")]
    Kubernetes,

    #[value(name = "ansible")]
    Ansible,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RulesConfig {
    pub(crate) rules: Vec<RuleDefinition>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RuleDefinition {
    pub(crate) code: String,
    pub(crate) file_type: FileType,
    pub(crate) category: String,
    pub(crate) severity: Severity,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) recommendation: String,
    pub(crate) example: Example,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Example {
    pub(crate) code: String,
}
