use crate::{
    analyzers::dockerfile::{
        context::AnalysisContext,
        rules::{df001::DF001, df004::DF004, df011::DF011},
    },
    core::issue::Issue,
};

pub (crate) fn execute_rules(ctx: &AnalysisContext) -> Vec<Issue> {
    let mut issues = Vec::new();

    if let Some(issue) = DF001::check(ctx) {
        issues.push(issue);
    }

    if let Some(issue) = DF004::check(ctx) {
        issues.push(issue);
    }

    if let Some(issue) = DF011::check(ctx) {
        issues.push(issue);
    }

    issues
}
