use crate::core::issue::Issue;
use dockerfile_parser::Instruction;

pub trait DockerfileRule: Send + Sync {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue>;
}
