use crate::syntax::checked::{SyntaxElem, RuleSet};

pub(crate) fn convert(ruleset: &RuleSet) -> String {
    ruleset
        .0
        .iter()
        .map(|rule| {
            let left = rule.left;
            let rights = rule.rights.iter().map(Into::<String>::into).collect::<Vec<String>>();
            let right = rights.join(" ");
            format!("{}: {}", left, right)
        })
        .collect::<Vec<String>>()
        .join(";\n") + ";"
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
            "top: top \"A\";",
            "top: \"A\";",
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
