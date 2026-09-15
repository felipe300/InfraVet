use crate::core::issue::Issue;
use dockerfile_parser::Instruction;

pub (crate) trait DockerfileRule {
    fn check(&self, instruction: &Instruction, content: &str, line: usize) -> Option<Issue>;
}
