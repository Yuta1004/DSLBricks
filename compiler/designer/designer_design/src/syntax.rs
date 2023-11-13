pub(crate) mod checked;
pub(crate) mod unchecked;

pub use unchecked::{Rule, RuleSet, SyntaxElem};

pub fn check(uc_ruleset: unchecked::RuleSet) -> (Vec<&'static str>, Vec<String>, checked::RuleSet) {
    // 1. Mark rules
    let marked_uc_ruleset = mark(uc_ruleset);

    // 2. Convert unchecked::Rule into check::Rule
    let ruleset = convert(marked_uc_ruleset);

    // 3-1. Collect token defs
    let token_defs = collect_token_defs(&ruleset);

    // 3-2. Collect syntax defs
    let syntax_defs = collect_syntax_defs(&ruleset);

    (token_defs, syntax_defs, ruleset)
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

fn collect_token_defs(ruleset: &checked::RuleSet) -> Vec<&'static str> {
    ruleset
        .0
        .iter()
        .flat_map(|rule| rule.rights.iter())
        .filter_map(|rule| {
            if let checked::SyntaxElem::Term(regex) = rule {
                Some(*regex)
            } else {
                None
            }
        })
        .collect()
}

fn collect_syntax_defs(ruleset: &checked::RuleSet) -> Vec<String> {
    ruleset
        .0
        .iter()
        .map(|rule| rule.name.clone())
        .collect()
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
        let (_, _, ruleset) = super::check(uc_ruleset);

        assert_eq!(format!("{:?}", except), format!("{:?}", ruleset),)
    }
}
