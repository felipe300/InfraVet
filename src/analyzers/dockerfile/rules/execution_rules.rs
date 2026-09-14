use crate::{
    analyzers::dockerfile::{
        context::AnalysisContext,
        rules::{df001::DF001, df004::DF004, df006::DF006},
    },
    core::issue::Issue,
};

pub fn execute_rules(content: &str, ctx: &AnalysisContext) -> Vec<Issue> {
    let mut issues = Vec::new();

    if let Some(issue) = DF006::check(content) {
        issues.push(issue);
    }

    if let Some(issue) = DF001::check(ctx) {
        issues.push(issue);
    }

    if let Some(issue) = DF004::check(ctx) {
        issues.push(issue);
    }

    issues
}
