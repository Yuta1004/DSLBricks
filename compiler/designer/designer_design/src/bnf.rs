use crate::syntax::checked::{SyntaxElem, Rule, RuleSet};

pub(crate) fn convert(ruleset: &RuleSet) -> String {
    ruleset
        .0
        .iter()
        .map(Into::<String>::into)
        .collect::<Vec<String>>()
        .join(";\n") + ";"
}

impl From<&Rule> for String {
    fn from(rule: &Rule) -> Self {
        let left = rule.left;
        let rights = rule.rights.iter().map(Into::<String>::into).collect::<Vec<String>>();
        let right = rights.join(" ");

        format!("{}: {} $ IgnoredRule", left, right)
    }
}

impl From<&SyntaxElem> for String {
    fn from(value: &SyntaxElem) -> Self {
        match value {
            SyntaxElem::Term(s) => format!("\"{}\"", s),
            SyntaxElem::NonTerm(s) => format!("{}", s),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::checked::{SyntaxElem, Rule};
    use super::convert;

    #[test]
    fn bnf() {
        let except = vec![
            "top: top \"A\" $ IgnoredRule;",
            "top: \"A\" $ IgnoredRule;",
        ];

        let ruleset = vec![
            Rule::from(
                ("top", vec![SyntaxElem::NonTerm("top"), SyntaxElem::Term("A")])
            ),
            Rule::from(
                ("top", vec![SyntaxElem::Term("A")])
            ),
        ].into();

        let result = convert(&ruleset)
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .all(|(line, except)| line == except);
        assert!(result)
    }
}
