pub(crate) mod checked;
pub(crate) mod unchecked;

pub use unchecked::{SyntaxElem, Rule, RuleSet};

pub fn check(uc_ruleset: unchecked::RuleSet) -> checked::RuleSet {
    uc_ruleset
        .into_iter()
        .map(|(left, rights)| {
            let rights = rights.into_iter().map(checked::SyntaxElem::from).collect();
            (left, rights)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::{unchecked, checked};

    #[test]
    fn check_simple() {
        let except = vec![
            ("top", vec![checked::SyntaxElem::NonTerm("top"), checked::SyntaxElem::Term("A")]),
            ("top", vec![checked::SyntaxElem::Term("A")]),
        ];

        let uc_ruleset = vec![
            ("top", vec![unchecked::SyntaxElem::NonTerm("top"), unchecked::SyntaxElem::Term("A")]),
            ("top", vec![unchecked::SyntaxElem::Term("A")]),
        ];
        let ruleset = super::check(uc_ruleset);

        assert_eq!(
            format!("{:?}", except),
            format!("{:?}", ruleset),
        )
    }
}
