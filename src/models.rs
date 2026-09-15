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

#[derive(Debug, PartialEq)]
pub(crate) enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RulesConfig {
    pub(crate) rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Rule {
    pub(crate) code: String,
    pub(crate) category: String,
    pub(crate) severity: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) recommendation: String,
    pub(crate) example: Example,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Example {
    pub(crate) dockerfile: String,
}
