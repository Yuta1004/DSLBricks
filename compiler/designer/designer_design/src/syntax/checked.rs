use crate::syntax::unchecked;

#[derive(Debug)]
pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
}

impl From<unchecked::SyntaxElem> for SyntaxElem {
    fn from(selem: unchecked::SyntaxElem) -> Self {
        match selem {
            unchecked::SyntaxElem::Term(s) => SyntaxElem::Term(s),
            unchecked::SyntaxElem::NonTerm(s) => SyntaxElem::NonTerm(s),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub(crate) name: String,
    pub(crate) left: &'static str,
    pub(crate) rights: Vec<SyntaxElem>,
}

impl<T> From<(T, &'static str, Vec<SyntaxElem>)> for Rule
where
    T: Into<String>,
{
    fn from((name, left, rights): (T, &'static str, Vec<SyntaxElem>)) -> Self {
        Rule {
            name: name.into(),
            left,
            rights
        }
    }
}

#[derive(Debug)]
pub struct RuleSet(pub(crate) Vec<Rule>);

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        RuleSet(rules)
    }
}

impl RuleSet {
    pub fn token_defs(&self) -> Vec<&'static str> {
        self.0
            .iter()
            .flat_map(|rule| rule.rights.iter())
            .filter_map(|rule| {
                if let SyntaxElem::Term(regex) = rule {
                    Some(*regex)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn syntax_defs(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|rule| rule.name.clone())
            .collect()
    }
}
