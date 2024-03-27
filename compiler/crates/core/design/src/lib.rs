pub mod syntax;
pub mod constraint {
    pub use constraint_ctime as ctime;
    pub use constraint_rtime as rtime;
}

use std::hash::Hash;

use syntax::{checked, unchecked};

pub struct DSLDesign {
    pub name: &'static str,
    pub ruleset: checked::RuleSet,
}

impl DSLDesign {
    pub fn try_from(dsl: impl DSLGeneratable) -> anyhow::Result<Self> {
        Ok(DSLDesign {
            name: dsl.name(),
            ruleset: syntax::check(dsl.design())?,
        })
    }
}

pub trait DSLGeneratable {
    fn name(&self) -> &'static str;
    fn start(&self) -> &'static str;
    fn design(&self) -> unchecked::RuleSet;
}

impl Hash for dyn DSLGeneratable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

impl PartialEq for dyn DSLGeneratable {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn DSLGeneratable {}
