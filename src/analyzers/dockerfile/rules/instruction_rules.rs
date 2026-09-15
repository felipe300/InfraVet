use crate::core::rule::DockerfileRule;

use super::{df002, df003, df005, df008, df009, df010};

pub (crate) fn instruction_rules() -> Vec<Box<dyn DockerfileRule>> {
    vec![
        Box::new(df002::DF002),
        Box::new(df003::DF003),
        Box::new(df005::DF005),
        Box::new(df008::DF008),
        Box::new(df009::DF009),
        Box::new(df010::DF010),
    ]
}
