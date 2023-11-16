mod bnf;
pub mod syntax;

use std::fmt::Debug;

use syntax::{checked, unchecked};

pub trait DSLGeneratable
where
    Self: Debug,
{
    fn name(&self) -> &'static str;
    fn start(&self) -> &'static str;
    fn design(&self) -> unchecked::RuleSet;
}

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

    pub fn token_defs<'a>(&'a self) -> Vec<(&'a String, &'static str)> {
        self.syntax.token_defs()
    }

    pub fn syntax_defs(&self) -> Vec<String> {
        self.syntax.syntax_defs()
    }

    pub fn bnf(&self) -> String {
        bnf::gen(&self.syntax)
    }
}
