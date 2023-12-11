use std::collections::HashSet;
use std::rc::Rc;

use crate::DSLGeneratable;

#[derive(Clone)]
pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(Rc<dyn DSLGeneratable>),
}

#[derive(Clone)]
pub struct Rule {
    pub(crate) namespace: &'static str,
    pub(crate) left: &'static str,
    pub(crate) rights: Vec<SyntaxElem>,
}

impl From<(&'static str, Vec<SyntaxElem>)> for Rule {
    fn from((left, rights): (&'static str, Vec<SyntaxElem>)) -> Self {
        Rule {
            namespace: "",
            left,
            rights,
        }
    }
}

impl From<(&'static str, Rule)> for Rule {
    fn from((namespace, rule): (&'static str, Rule)) -> Self {
        Rule {
            namespace,
            left: rule.left,
            rights: rule.rights,
        }
    }
}

#[derive(Clone)]
pub struct RuleSet(pub(crate) Vec<Rule>);

impl From<(&'static str, Vec<Rule>)> for RuleSet {
    fn from((namespace, rules): (&'static str, Vec<Rule>)) -> Self {
        let rules = rules
            .into_iter()
            .map(|rule| Rule::from((namespace, rule)))
            .collect();

        RuleSet(rules)
    }
}

impl RuleSet {
    pub(crate) fn expand(mut self) -> RuleSet {
        let expanded = self
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
                    .collect::<Vec<&Rc<dyn DSLGeneratable>>>()
            })
            .collect::<HashSet<&Rc<dyn DSLGeneratable>>>()
            .into_iter()
            .flat_map(|design| design.design().expand().0)
            .collect::<Vec<Rule>>();

        self.0.extend(expanded);

        self
    }
}
