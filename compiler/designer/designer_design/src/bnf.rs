use crate::syntax::checked::{Rule, RuleSet, SyntaxElem};

pub(crate) fn convert(ruleset: &RuleSet) -> String {
    ruleset
        .0
        .iter()
        .map(Into::<String>::into)
        .collect::<Vec<String>>()
        .join(";\n")
        + ";"
}

impl From<&Rule> for String {
    fn from(rule: &Rule) -> Self {
        let left = rule.left;
        let rights = rule
            .rights
            .iter()
            .map(Into::<String>::into)
            .collect::<Vec<String>>();
        let right = rights.join(" ");

        format!("{}: {} $ IgnoredRule", left, right)
    }
}

impl From<&SyntaxElem> for String {
    fn from(value: &SyntaxElem) -> Self {
        match value {
            SyntaxElem::Term(s) => format!("\"{}\"", s),
            SyntaxElem::NonTerm(s) => format!("{}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::convert;
    use crate::syntax::checked::{Rule, SyntaxElem};

    #[test]
    fn bnf() {
        let except = vec!["top: top \"A\" $ IgnoredRule;", "top: \"A\" $ IgnoredRule;"];

        let ruleset = vec![
            Rule::from((
                "top_0",
                "top",
                vec![SyntaxElem::NonTerm("top"), SyntaxElem::Term("A")],
            )),
            Rule::from((
                "top_1",
                "top",
                vec![SyntaxElem::Term("A")]
            )),
        ]
        .into();

        let result = convert(&ruleset)
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .all(|(line, except)| line == except);
        assert!(result)
    }
}
