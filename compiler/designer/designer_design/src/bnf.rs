use crate::syntax::checked::{Rule, RuleSet, SyntaxElem};

pub(crate) fn gen(ruleset: &RuleSet) -> String {
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
        let name = &rule.name;
        let left = rule.left;
        let rights = rule
            .rights
            .iter()
            .map(Into::<String>::into)
            .collect::<Vec<String>>();
        let right = rights.join(" ");

        format!("{}: {} $ {}", left, right, name)
    }
}

impl From<&SyntaxElem> for String {
    fn from(value: &SyntaxElem) -> Self {
        match value {
            SyntaxElem::Term(id, _) => format!("\"{}\"", id),
            SyntaxElem::NonTerm(left) => format!("{}", left),
        }
    }
}

#[cfg(test)]
mod test {
    use super::gen;
    use crate::syntax::checked::{Rule, SyntaxElem};

    #[test]
    fn bnf() {
        let except = vec!["top: top \"token_1\" $ top_0;", "top: \"token_1\" $ top_1;"];

        let ruleset = vec![
            Rule::from((
                "top_0",
                "top",
                vec![
                    SyntaxElem::NonTerm("top"),
                    SyntaxElem::Term("token_1".to_string(), "A")
                ],
            )),
            Rule::from((
                "top_1",
                "top",
                vec![SyntaxElem::Term("token_1".to_string(), "A")]
            )),
        ]
        .into();

        gen(&ruleset)
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .for_each(|(line, except)| assert_eq!(line, except));
    }
}
