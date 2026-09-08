use std::{fs, path::Path};

use anyhow::{Context, Result};
use dockerfile_parser::{Dockerfile, Instruction};

use crate::{
    analyzer::issue::{Issue, RuleId, Severity},
    utils::utils::uses_latest_tag,
};

fn offset_to_line(content: &str, offset: usize) -> usize {
    content[..offset.min(content.len())].lines().count().max(1)
}

pub fn analyze_dockerfile(path: &Path) -> Result<Vec<Issue>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Unable to read file: {}", path.display()))?;

    // DF006 - Empty Dockerfile
    if content.trim().is_empty() {
        return Ok(vec![Issue {
            rule: RuleId::new("DF006"),
            line: 1,
            message: "Dockerfile is empty.".into(),
            severity: Severity::Error,
        }]);
    }

    // Parse Dockerfile
    let dockerfile = match Dockerfile::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            // DF007 - Invalid Dockerfile syntax
            return Ok(vec![Issue {
                rule: RuleId::new("DF007"),
                line: 1,
                message: format!("Invalid Dockerfile syntax: {}", err),
                severity: Severity::Error,
            }]);
        }
    };

    let mut issues = Vec::new();
    let mut has_from = false;

    let mut stages_count = 0;
    let mut user_in_current_stage = false;

    // Inspect each instruction
    for instruction in dockerfile.instructions {
        let line = offset_to_line(&content, instruction.span().start);

        match instruction {
            Instruction::From(from) => {
                has_from = true;
                stages_count += 1;
                user_in_current_stage = false;

                // DF003 - latest image tag
                let span = from.span;
                let from_str = &content[span.start..span.end];

                if uses_latest_tag(from_str) {
                    issues.push(Issue {
                        rule: RuleId::new("DF003"),
                        line,
                        message: "Base image uses the 'latest' tag. Consider using a specific version or a slim variant.".into(),
                        severity: Severity::Info,
                    });
                }
            }

            Instruction::Run(run) => {
                let span = run.span;
                let command = &content[span.start..span.end];

                // DF005 - apt-get update without apt-get install
                if command.contains("apt-get update") && !command.contains("apt-get install") {
                    issues.push(Issue {
                        rule: RuleId::new("DF005"),
                        line,
                        message: "'apt-get update' is used without 'apt-get install' in the same RUN instruction.".into(),
                        severity: Severity::Warning,
                    });
                }
            }

            Instruction::Copy(copy) => {
                let span = copy.span;
                let copy_str = &content[span.start..span.end];

                // DF002 - COPY . .
                if copy_str.contains(". .") {
                    issues.push(Issue {
                        rule: RuleId::new("DF002"),
                        line,
                        message: "COPY . . copies the entire build context. Consider copying only the required files or directories.".into(),
                        severity: Severity::Warning,
                    });
                }
            }

            Instruction::Misc(misc) => {
                if misc
                    .instruction
                    .content
                    .as_str()
                    .eq_ignore_ascii_case("User")
                {
                    user_in_current_stage = true;
                }
            }

            _ => {}
        }
    }

    // DF001 - Missing FROM
    if !has_from {
        issues.push(Issue {
            rule: RuleId::new("DF001"),
            line: 1,
            message: "Missing FROM instruction. A Dockerfile must define at least one build stage with FROM.".into(),
            severity: Severity::Error,
        });
    }

    // DF004 - Missing USER
    if has_from && !user_in_current_stage {
        let message = if stages_count > 1 {
            "Missing USER instruction in the final runtime stage. The container will run as root."
        } else {
            "Missing USER instruction. The container will run as root."
        };

        issues.push(Issue {
            rule: RuleId::new("DF004"),
            line: 1,
            message: message.into(),
            severity: Severity::Warning,
        });
    }

    Ok(issues)
}
