use std::rc::Rc;
use std::collections::HashSet;

use crate::DSLGeneratable;

#[derive(Clone)]
pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(Rc<Box<dyn DSLGeneratable>>),
}

#[derive(Clone)]
pub struct Rule {
    pub(crate) left: &'static str,
    pub(crate) rights: Vec<SyntaxElem>,
}

impl From<(&'static str, Vec<SyntaxElem>)> for Rule {
    fn from((left, rights): (&'static str, Vec<SyntaxElem>)) -> Self {
        Rule { left, rights }
    }
}

#[derive(Clone)]
pub struct RuleSet(pub(crate) Vec<Rule>);

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        RuleSet(rules)
    }
}

impl RuleSet {
    pub(crate) fn expand(mut self) -> RuleSet {
        let expanded= self
            .0
            .iter()
            .flat_map(|rule| {
                rule.rights
                    .iter()
                    .flat_map(|selem| {
                        if let SyntaxElem::Hole(design) = selem {
                            Some(design)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&Rc<Box<dyn DSLGeneratable>>>>()
            })
            .collect::<HashSet<&Rc<Box<dyn DSLGeneratable>>>>()
            .into_iter()
            .flat_map(|design| design.design().expand().0)
            .collect::<Vec<Rule>>();

        self.0.extend(expanded);

        self
    }
}
