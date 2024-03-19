pub mod macros;
pub(crate) mod checked;
pub(crate) mod unchecked;

use std::collections::HashMap;

pub use macros::rule;
pub use unchecked::{Rule, RuleSet, SyntaxElem};

pub fn check(uc_ruleset: unchecked::RuleSet) -> anyhow::Result<checked::RuleSet> {
    let mut context = CheckContext::default();

    let uc_ruleset = uc_ruleset.expand();
    let token_set = collect_tokens(&mut context, &uc_ruleset);
    let marked_uc_rules = mark(&mut context, uc_ruleset);

    convert(marked_uc_rules, token_set)
}

#[derive(Debug, Default)]
struct CheckContext {
    token_id: i32,
    rule_id: i32,
}

impl CheckContext {
    fn issue_token_id(&mut self) -> i32 {
        let token_id = self.token_id;
        self.token_id += 1;
        token_id
    }

    fn issue_rule_id(&mut self) -> i32 {
        let rule_id = self.rule_id;
        self.rule_id += 1;
        rule_id
    }
}

fn collect_tokens(
    context: &mut CheckContext,
    uc_ruleset: &unchecked::RuleSet,
) -> HashMap<&'static str, String> {
    let mut token_set = HashMap::new();
    for rule in uc_ruleset.0.iter() {
        for selem in rule.rights.iter() {
            if let unchecked::SyntaxElem::Term(regex) = selem {
                let id = format!("token_{}", context.issue_token_id());
                token_set.insert(*regex, id);
            }
        }
    }

    token_set
}

fn mark(
    context: &mut CheckContext,
    uc_ruleset: unchecked::RuleSet,
) -> Vec<(String, unchecked::Rule)> {
    let mut marked_uc_ruleset = vec![];
    for rule in uc_ruleset.0 {
        let id = format!("rule_{}", context.issue_rule_id());
        marked_uc_ruleset.push((id, rule));
    }

    marked_uc_ruleset
}

fn convert(
    marked_uc_rules: Vec<(String, unchecked::Rule)>,
    token_set: HashMap<&'static str, String>,
) -> anyhow::Result<checked::RuleSet> {
    let convert = |namespace: &str, rule: unchecked::SyntaxElem| match rule {
        unchecked::SyntaxElem::Term(regex) => {
            let id = token_set.get(regex).unwrap();
            checked::SyntaxElem::Term(id.clone(), regex)
        }
        unchecked::SyntaxElem::NonTerm(left) => {
            let left = format!("{}.{}", namespace, left);
            checked::SyntaxElem::NonTerm(left)
        }
        unchecked::SyntaxElem::Hole(design) => {
            let left = format!("{}.{}", design.name(), design.start());
            checked::SyntaxElem::NonTerm(left)
        }
    };

    let ruleset = marked_uc_rules
        .into_iter()
        .map(|(id, rule)| {
            let left = format!("{}.{}", rule.namespace, rule.left);
            let rights = rule
                .rights
                .into_iter()
                .map(|selem| convert(rule.namespace, selem))
                .collect();
            checked::Rule::from((id, left, rights))
        })
        .collect::<Vec<checked::Rule>>()
        .into();

    Ok(ruleset)
}

#[cfg(test)]
mod test {
    use super::{checked, unchecked};

    #[test]
    fn check_simple() {
        let except: checked::RuleSet = vec![
            checked::Rule::from((
                "rule_0",
                ".top",
                vec![
                    checked::SyntaxElem::NonTerm(".top".to_string()),
                    checked::SyntaxElem::Term("token_1".to_string(), "A"),
                ],
            )),
            checked::Rule::from((
                "rule_1",
                ".top",
                vec![checked::SyntaxElem::Term("token_1".to_string(), "A")],
            )),
        ]
        .into();

        let uc_rules = vec![
            unchecked::Rule::from((
                "top",
                vec![
                    unchecked::SyntaxElem::NonTerm("top"),
                    unchecked::SyntaxElem::Term("A"),
                ],
            )),
            unchecked::Rule::from(("top", vec![unchecked::SyntaxElem::Term("A")])),
        ];
        let uc_ruleset = ("", uc_rules).into();
        let ruleset = super::check(uc_ruleset).unwrap();

        assert!(except == ruleset);
    }
}
