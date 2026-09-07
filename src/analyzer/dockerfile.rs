use std::{fs, path::Path};

use anyhow::{Context, Result};
use dockerfile_parser::{Dockerfile, Instruction};

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Issue {
    pub line: usize,
    pub message: String,
    pub severity: Severity,
}

fn offset_to_line(content: &str, offset: usize) -> usize {
    content[..offset.min(content.len())].lines().count().max(1)
}

pub fn analyze_dockerfile(path: &Path) -> Result<Vec<Issue>> {
    // read file
    let content = fs::read_to_string(path)
        .with_context(|| format!("No se puede leer el archivo: {}", path.display()))?;

    // check if file is empty
    if content.trim().is_empty() {
        return Ok(vec![Issue {
            line: 1,
            message: "El archivo Dockerfile está vacío.".into(),
            severity: Severity::Error,
        }]);
    }

    // parse the content to AST
    let dockerfile = match Dockerfile::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            // Detect Syntax Error
            return Ok(vec![Issue {
                line: 1,
                message: format!("Error al parsear la sintanxis de Dockerfile: {}", err),
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
            Instruction::From(_) => {
                has_from = true;
                stages_count += 1;
                user_in_current_stage = false;
            }

            Instruction::Run(run) => {
                let span = run.span;
                let command = &content[span.start..span.end];

                if command.contains("apt-get update") && !command.contains("apt-get install") {
                    issues.push(Issue {
                        line,
                        message: "Se ejecuta 'apt-get update' sin 'apt-get install' en la misma instrucción RUN.".into(),
                        severity: Severity::Warning,
                    });
                }
            }

            Instruction::Copy(copy) => {
                let span = copy.span;
                let copy_str = &content[span.start..span.end];

                if copy_str.contains(". .") {
                    issues.push(Issue {
                        line,
                        message: "Uso de 'COPY . .'. Considera copiar archivos o directorios específicos.".into(),
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

    if !has_from {
        issues.push(Issue {
            line: 1,
            message: "Falta la instruccion 'From' base.".into(),
            severity: Severity::Error,
        });
    }

    if has_from && !user_in_current_stage {
        let msg = if stages_count > 1 {
            "Falta la instrucción 'USER' en la etapa final (runtime). El contenedor final correrá como 'root'."
        } else {
            "Falta la instruction 'User'. El contenedor utilizará 'root' como usuario."
        };

        issues.push(Issue {
            line: 1,
            message: msg.into(),
            severity: Severity::Warning,
        });
    }

    Ok(issues)
}
