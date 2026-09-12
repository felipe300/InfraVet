pub mod df001;
pub mod df002;
pub mod df003;
pub mod df004;
pub mod df005;
pub mod df006;
pub mod df007;
pub mod df008;

use crate::core::rule::DockerfileRule;

pub fn all_rules() -> Vec<Box<dyn DockerfileRule>> {
    vec![
        Box::new(df002::DF002),
        Box::new(df003::DF003),
        Box::new(df005::DF005),
    ]
}
