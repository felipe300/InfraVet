use anyhow::{Context, Result};
use dockerfile_parser::Dockerfile;
use std::{fs, path::Path};

use crate::analyzers::dockerfile::context::AnalysisContext;
use crate::analyzers::dockerfile::rules::{
    all_rules, df001::DF001, df004::DF004, df006::DF006, df007::DF007,
};
use crate::core::issue::Issue;

fn offset_to_line(content: &str, offset: usize) -> usize {
    content[..offset.min(content.len())].lines().count().max(1)
}

pub fn analyze_dockerfile(path: &Path) -> Result<Vec<Issue>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Unable to read file: {}", path.display()))?;

    // DF006 - Archivo vacío
    if let Some(issue) = DF006::check(&content) {
        return Ok(vec![issue]);
    }

    // Parseo y DF007 - Error de sintaxis
    let dockerfile = match Dockerfile::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => return Ok(vec![DF007::check(&err.to_string())]),
    };

    let mut issues = Vec::new();
    let mut ctx = AnalysisContext::new();
    let rules = all_rules();

    // Análisis instrucción por instrucción
    for instruction in &dockerfile.instructions {
        let line = offset_to_line(&content, instruction.span().start);

        ctx.update(instruction);

        for rule in &rules {
            if let Some(issue) = rule.check(instruction, &content, line) {
                issues.push(issue);
            }
        }
    }

    // Reglas de estado global
    if let Some(issue) = DF001::check(&ctx) {
        issues.push(issue);
    }

    if let Some(issue) = DF004::check(&ctx) {
        issues.push(issue);
    }

    Ok(issues)
}
