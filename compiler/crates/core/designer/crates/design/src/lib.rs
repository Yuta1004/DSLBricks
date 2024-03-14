pub mod syntax;

use std::hash::Hash;

use syntax::{checked, unchecked};

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

pub struct DSLDesign {
    pub name: &'static str,
    syntax: checked::RuleSet,
}

impl DSLDesign {
    pub fn from<T: DSLGeneratable>(def: T) -> anyhow::Result<Self> {
        Ok(DSLDesign {
            name: def.name(),
            syntax: syntax::check(def.design())?,
        })
    }

    pub fn token_defs(&self) -> Vec<(&String, &'static str)> {
        self.syntax.token_defs()
    }

    pub fn syntax_defs(&self) -> Vec<String> {
        self.syntax.syntax_defs()
    }

    pub fn bnf(&self) -> String {
        (&self.syntax).into()
    }
}