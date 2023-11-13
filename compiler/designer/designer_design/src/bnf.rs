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
            SyntaxElem::Term(s) => format!("\"{}\"", s),
            SyntaxElem::NonTerm(s) => format!("{}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::gen;
    use crate::syntax::checked::{Rule, SyntaxElem};

    #[test]
    fn bnf() {
        let except = vec!["top: top \"A\" $ top_1;", "top: \"A\" $ top_2;"];

        let ruleset = vec![
            Rule::from((
                "top_1",
                "top",
                vec![SyntaxElem::NonTerm("top"), SyntaxElem::Term("A")],
            )),
            Rule::from(("top_2", "top", vec![SyntaxElem::Term("A")])),
        ]
        .into();

        let result = gen(&ruleset)
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .all(|(line, except)| line == except);
        assert!(result)
    }
}
