pub(crate) mod checked;
pub(crate) mod unchecked;

pub use unchecked::{Rule, RuleSet, SyntaxElem};

pub fn check(uc_ruleset: unchecked::RuleSet) -> anyhow::Result<checked::RuleSet> {
    // 1. Mark rules
    let marked_uc_ruleset = mark(uc_ruleset);

    // 2. Convert unchecked::Rule into check::Rule
    let ruleset = convert(marked_uc_ruleset);

    Ok(ruleset)
}

fn mark(uc_ruleset: unchecked::RuleSet) ->  Vec<(String, unchecked::Rule)> {
    let mut id = 0;
    let mut marked_uc_ruleset = vec![];
    for rule in uc_ruleset.0.into_iter() {
        id += 1;
        let id = format!("{}_{}", rule.left, id);
        marked_uc_ruleset.push((id, rule));
    }

    marked_uc_ruleset
}

fn convert(marked_uc_ruleset: Vec<(String, unchecked::Rule)>) -> checked::RuleSet {
    marked_uc_ruleset
        .into_iter()
        .map(|(id, rule)| {
            let left = rule.left;
            let rights = rule
                .rights
                .into_iter()
                .map(checked::SyntaxElem::from)
                .collect();
            checked::Rule::from((id, left, rights))
        })
        .collect::<Vec<checked::Rule>>()
        .into()
}

#[cfg(test)]
mod test {
    use super::{checked, unchecked};

    #[test]
    fn check_simple() {
        let except: checked::RuleSet = vec![
            checked::Rule::from((
                "top_1",
                "top",
                vec![
                    checked::SyntaxElem::NonTerm("top"),
                    checked::SyntaxElem::Term("A"),
                ],
            )),
            checked::Rule::from((
                "top_2",
                "top",
                vec![checked::SyntaxElem::Term("A")]
            )),
        ]
        .into();

        let uc_ruleset: unchecked::RuleSet = vec![
            unchecked::Rule::from((
                "top",
                vec![
                    unchecked::SyntaxElem::NonTerm("top"),
                    unchecked::SyntaxElem::Term("A"),
                ],
            )),
            unchecked::Rule::from(("top", vec![unchecked::SyntaxElem::Term("A")])),
        ]
        .into();
        let ruleset = super::check(uc_ruleset).unwrap();

        assert_eq!(format!("{:?}", except), format!("{:?}", ruleset),)
    }
}
