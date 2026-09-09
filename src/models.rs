use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum FileType {
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
