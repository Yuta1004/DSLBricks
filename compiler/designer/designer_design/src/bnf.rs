use crate::syntax::checked::{SyntaxElem, RuleSet};

pub(crate) fn convert(ruleset: &RuleSet) -> String {
    ruleset
        .into_iter()
        .map(|(left, rights)| {
            let rights = rights.into_iter().map(Into::<String>::into).collect::<Vec<String>>();
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
    use crate::syntax::checked::SyntaxElem;
    use super::convert;

    #[test]
    fn bnf() {
        let except = vec![
            "top: top \"A\";",
            "top: \"A\";",
        ];

        let ruleset = vec![
            ("top", vec![SyntaxElem::NonTerm("top"), SyntaxElem::Term("A")]),
            ("top", vec![SyntaxElem::Term("A")]),
        ];

        let result = convert(&ruleset)
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .all(|(line, except)| line == except);
        assert!(result)
    }
}
