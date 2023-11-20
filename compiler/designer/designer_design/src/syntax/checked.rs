#[derive(PartialEq, Eq)]
pub enum SyntaxElem {
    Term(String, &'static str),
    NonTerm(&'static str),
}

impl Into<String> for &SyntaxElem {
    fn into(self) -> String {
        match self {
            SyntaxElem::Term(id, _) => format!("\"{}\"", id),
            SyntaxElem::NonTerm(left) => format!("{}", left),
        }
    }
}

#[derive(PartialEq, Eq)]
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
            rights,
        }
    }
}

impl Into<String> for &Rule {
    fn into(self) -> String {
        let name = &self.name;
        let left = self.left;
        let rights = self
            .rights
            .iter()
            .map(Into::<String>::into)
            .collect::<Vec<String>>();
        let right = rights.join(" ");

        format!("{}: {} $ {}", left, right, name)
    }
}

#[derive(PartialEq, Eq)]
pub struct RuleSet(pub(crate) Vec<Rule>);

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        RuleSet(rules)
    }
}

impl Into<String> for &RuleSet {
    fn into(self) -> String {
        self.0
            .iter()
            .map(Into::<String>::into)
            .collect::<Vec<String>>()
            .join(";\n")
            + ";"
    }
}

impl RuleSet {
    pub fn token_defs<'a>(&'a self) -> Vec<(&'a String, &'static str)> {
        self.0
            .iter()
            .flat_map(|rule| rule.rights.iter())
            .filter_map(|rule| {
                if let SyntaxElem::Term(id, regex) = rule {
                    Some((id, *regex))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn syntax_defs(&self) -> Vec<String> {
        self.0.iter().map(|rule| rule.name.clone()).collect()
    }
}
