pub(crate) mod checked;
pub(crate) mod unchecked;

pub use unchecked::{SyntaxElem, Rule, RuleSet};

pub fn check(uc_ruleset: unchecked::RuleSet) -> checked::RuleSet {
    uc_ruleset
        .0
        .into_iter()
        .map(|rule| {
            let left = rule.left;
            let rights = rule.rights.into_iter().map(checked::SyntaxElem::from).collect();
            checked::Rule::from((left, rights))
        })
        .collect::<Vec<checked::Rule>>()
        .into()
}

#[cfg(test)]
mod test {
    use super::{unchecked, checked};

    #[test]
    fn check_simple() {
        let except: checked::RuleSet = vec![
            checked::Rule::from(
                ("top", vec![checked::SyntaxElem::NonTerm("top"), checked::SyntaxElem::Term("A")])
            ),
            checked::Rule::from(
                ("top", vec![checked::SyntaxElem::Term("A")])
            ),
        ].into();

        let uc_ruleset: unchecked::RuleSet = vec![
            unchecked::Rule::from(
                ("top", vec![unchecked::SyntaxElem::NonTerm("top"), unchecked::SyntaxElem::Term("A")])
            ),
            unchecked::Rule::from(
                ("top", vec![unchecked::SyntaxElem::Term("A")])
            ),
        ].into();
        let ruleset = super::check(uc_ruleset);

        assert_eq!(
            format!("{:?}", except),
            format!("{:?}", ruleset),
        )
    }
}
