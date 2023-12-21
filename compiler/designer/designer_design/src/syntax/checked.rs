use std::collections::HashSet;

#[derive(PartialEq, Eq)]
pub enum SyntaxElem {
    Term(String, &'static str),
    NonTerm(String),
}

impl From<&SyntaxElem> for String {
    fn from(selem: &SyntaxElem) -> String {
        match selem {
            SyntaxElem::Term(id, _) => format!("\"{}\"", id),
            SyntaxElem::NonTerm(left) => left.to_string(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Rule {
    pub(crate) name: String,
    pub(crate) left: String,
    pub(crate) rights: Vec<SyntaxElem>,
}

impl<T> From<(T, T, Vec<SyntaxElem>)> for Rule
where
    T: Into<String>,
{
    fn from((name, left, rights): (T, T, Vec<SyntaxElem>)) -> Self {
        Rule {
            name: name.into(),
            left: left.into(),
            rights,
        }
    }
}

impl From<&Rule> for String {
    fn from(rule: &Rule) -> String {
        let name = &rule.name;
        let left = &rule.left;
        let rights = rule
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

impl From<&RuleSet> for String {
    fn from(ruleset: &RuleSet) -> String {
        ruleset.0
            .iter()
            .map(Into::<String>::into)
            .collect::<Vec<String>>()
            .join(";\n")
            + ";"
    }
}

impl RuleSet {
    pub fn token_defs(&self) -> Vec<(&String, &'static str)> {
        let score = |regex: &str| {
            let mut score = 0;
            score += regex.len();
            score += if regex.contains(['*', '+']) { 100 } else { 0 };
            score
        };

        let mut tokens = self.0
            .iter()
            .flat_map(|rule| rule.rights.iter())
            .filter_map(|rule| {
                if let SyntaxElem::Term(id, regex) = rule {
                    Some((id, *regex))
                } else {
                    None
                }
            })
            .collect::<HashSet<(&String, &str)>>()
            .into_iter()
            .map(|(id, regex)| (score(regex), (id, regex)))
            .collect::<Vec<(usize, (&String, &str))>>();
        tokens.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        tokens.into_iter()
            .map(|(_, item)| item)
            .collect()
    }

    pub fn syntax_defs(&self) -> Vec<String> {
        self.0.iter().map(|rule| rule.name.clone()).collect()
    }
}
