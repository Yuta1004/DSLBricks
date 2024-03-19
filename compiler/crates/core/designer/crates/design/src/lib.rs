pub mod syntax;

use std::hash::Hash;

use syntax::{checked, unchecked};

pub struct DSLDesign {
    pub name: &'static str,
    pub ruleset: checked::RuleSet,
}

pub trait DSLGeneratable {
    fn name(&self) -> &'static str;
    fn start(&self) -> &'static str;
    fn design(&self) -> unchecked::RuleSet;

    fn try_into(self) -> anyhow::Result<DSLDesign>
    where
        Self: Sized,
    {
        Ok(DSLDesign {
            name: self.name(),
            ruleset: syntax::check(self.design())?,
        })
    }
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
