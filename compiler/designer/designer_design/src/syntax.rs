pub(crate) mod checked;
pub(crate) mod unchecked;

use std::collections::HashMap;

pub use unchecked::{Rule, RuleSet, SyntaxElem};

pub fn check(uc_ruleset: unchecked::RuleSet) -> anyhow::Result<checked::RuleSet> {
    // 1. Collect tokens
    let token_set = collect_tokens(&uc_ruleset);

    // 2. Mark rules
    let marked_uc_ruleset = mark(uc_ruleset);

    // 3. Convert unchecked::Rule into check::Rule
    let ruleset = convert(marked_uc_ruleset, token_set);

    Ok(ruleset)
}

fn collect_tokens(uc_ruleset: &unchecked::RuleSet) -> HashMap<&'static str, String> {
    let mut id = 0;
    let mut token_set = HashMap::new();
    for rule in uc_ruleset.0.iter() {
        for selem in rule.rights.iter() {
            if let SyntaxElem::Term(regex) = selem {
                id += 1;
                let id = format!("token_{}", id);
                token_set.insert(*regex, id);
            }
        }
    }

    token_set
}

fn mark(uc_ruleset: unchecked::RuleSet) -> Vec<(String, unchecked::Rule)> {
    let mut id = 0;
    let mut marked_uc_ruleset = vec![];
    for rule in uc_ruleset.0 {
        id += 1;
        let id = format!("{}_{}", rule.left, id);
        marked_uc_ruleset.push((id, rule));
    }

    marked_uc_ruleset
}

fn convert(marked_uc_ruleset: Vec<(String, unchecked::Rule)>, token_set: HashMap<&'static str, String>) -> checked::RuleSet {
    let convert = |rule: unchecked::SyntaxElem| {
        match rule {
            unchecked::SyntaxElem::Term(regex) => {
                let id = token_set.get(regex).unwrap();
                checked::SyntaxElem::Term(id.clone(), regex)
            }
            unchecked::SyntaxElem::NonTerm(left) => {
                checked::SyntaxElem::NonTerm(left)
            }
            _ => unimplemented!()
        }
    };

    marked_uc_ruleset
        .into_iter()
        .map(|(id, rule)| {
            let left = rule.left;
            let rights = rule
                .rights
                .into_iter()
                .map(convert)
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
                    checked::SyntaxElem::Term("token_1".to_string(), "A"),
                ],
            )),
            checked::Rule::from((
                "top_2",
                "top",
                vec![checked::SyntaxElem::Term("token_1".to_string(), "A")]
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
