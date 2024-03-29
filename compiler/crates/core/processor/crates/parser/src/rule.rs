use std::collections::HashMap;
use std::hash::Hash;

use lexer::TokenSet;

#[derive(Debug)]
pub enum RuleElem<T: TokenSet> {
    NonTerm(String),
    Term(T),
    EOF,
}

impl<T: TokenSet> Hash for RuleElem<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RuleElem::NonTerm(s) => s.hash(state),
            RuleElem::Term(t) => t.hash(state),
            RuleElem::EOF => 0.hash(state),
        }
    }
}

impl<T: TokenSet> PartialEq for RuleElem<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleElem::NonTerm(s1), RuleElem::NonTerm(s2)) => s1 == s2,
            (RuleElem::Term(t1), RuleElem::Term(t2)) => t1 == t2,
            (RuleElem::EOF, RuleElem::EOF) => true,
            _ => false,
        }
    }
}

impl<T: TokenSet> Eq for RuleElem<T> {}

impl<T: TokenSet> RuleElem<T> {
    pub fn nonterm<U: Into<String>>(t: U) -> RuleElem<T> {
        RuleElem::NonTerm(t.into())
    }

    pub fn term(t: T) -> RuleElem<T> {
        RuleElem::Term(t)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Rule<T: TokenSet> {
    pub id: i32,
    pub left: RuleElem<T>,
    pub right: Vec<RuleElem<T>>,
}

impl<T: TokenSet> From<(RuleElem<T>, Vec<RuleElem<T>>)> for Rule<T> {
    fn from((left, right): (RuleElem<T>, Vec<RuleElem<T>>)) -> Self {
        Rule { id: 0, left, right }
    }
}

impl<T: TokenSet> Rule<T> {
    pub fn nonterms(&self) -> Vec<&RuleElem<T>> {
        let mut l_nonterms = vec![&self.left];
        let r_nonterms: Vec<&RuleElem<T>> = self
            .right
            .iter()
            .filter(|token| matches!(token, RuleElem::<T>::NonTerm(_)))
            .collect();
        l_nonterms.extend(r_nonterms);
        l_nonterms
    }

    pub fn terms(&self) -> Vec<&RuleElem<T>> {
        self.right
            .iter()
            .filter(|token| matches!(token, RuleElem::<T>::Term(_)))
            .collect()
    }
}

#[derive(Debug)]
pub struct RuleSet<T: TokenSet> {
    pub top: String,
    pub rules: Vec<Rule<T>>,
}

impl<T, U> From<(U, Vec<Rule<T>>)> for RuleSet<T>
where
    T: TokenSet,
    U: Into<String>,
{
    fn from((top, mut rules): (U, Vec<Rule<T>>)) -> Self {
        for (idx, rule) in rules.iter_mut().enumerate() {
            rule.id = idx as i32;
        }

        RuleSet {
            top: top.into(),
            rules,
        }
    }
}

impl<T: TokenSet> RuleSet<T> {
    pub fn nonterms(&self) -> Vec<&RuleElem<T>> {
        self.rules.iter().flat_map(|rule| rule.nonterms()).collect()
    }

    pub fn terms(&self) -> Vec<&RuleElem<T>> {
        self.rules.iter().flat_map(|rule| rule.terms()).collect()
    }

    pub fn find_rule(&self, target: &RuleElem<T>) -> Vec<&Rule<T>> {
        self.rules
            .iter()
            .filter(|rule| &rule.left == target)
            .collect()
    }

    pub fn first_set(&self) -> HashMap<&RuleElem<T>, Vec<&RuleElem<T>>> {
        // 1. Calc a null set
        let nulls_set = self.nulls_set();

        // 2. Initialize a first set
        let mut first_set: HashMap<&RuleElem<T>, Vec<&RuleElem<T>>> = HashMap::new();
        first_set.insert(&RuleElem::EOF, vec![&RuleElem::EOF]);
        self.terms().into_iter().for_each(|relem| {
            first_set.insert(relem, vec![relem]);
        });
        self.nonterms().into_iter().for_each(|relem| {
            first_set.insert(relem, vec![]);
        });

        // 3. List up candidates from a nonterm set
        let mut candidates = vec![];
        for nonterm in self.nonterms() {
            let rules = self.find_rule(nonterm);
            for rule in rules {
                for relem in &rule.right {
                    if &rule.left != relem {
                        candidates.push((nonterm, relem))
                    }
                    if !nulls_set.contains(&relem) {
                        break;
                    }
                }
            }
        }

        // 4. Find first set with recursive
        let mut updated = true;
        while updated {
            updated = false;
            for (nonterm, candidate) in &candidates {
                let found_elems: Vec<&RuleElem<T>> = first_set
                    .get(candidate)
                    .unwrap()
                    .iter()
                    .filter(|relem| !first_set.get(nonterm).unwrap().contains(relem))
                    .copied()
                    .collect();
                updated = !found_elems.is_empty();
                first_set
                    .get_mut(nonterm)
                    .unwrap()
                    .extend(found_elems.into_iter());
            }
        }

        first_set
    }

    fn nulls_set(&self) -> Vec<&RuleElem<T>> {
        // 1. Find null rules
        let mut nulls_set: Vec<&RuleElem<T>> = self
            .rules
            .iter()
            .filter(|rule| rule.right.is_empty())
            .map(|rule| &rule.left)
            .collect();

        // 2. Find null rules with recursive
        let mut updated = true;
        while updated {
            updated = false;
            for rule in &self.rules {
                if nulls_set.contains(&&rule.left) {
                    continue;
                } else if rule.right.iter().all(|relem| nulls_set.contains(&relem)) {
                    nulls_set.push(&rule.left);
                    updated = true;
                } else {
                    continue;
                }
            }
        }

        nulls_set
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use lexer::TokenSet;

    use crate::kind::LR1;
    use crate::rule::{Rule, RuleElem};
    use crate::syntax::{post, pre};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct VoidSemantics;

    impl<T, PreS> post::Syntax<T, PreS> for VoidSemantics
    where
        PreS: pre::Syntax<T, Self>,
        T: TokenSet,
    {
        fn mapping(
            _: PreS,
            _: Vec<(Option<Box<Self>>, Option<&str>)>,
        ) -> anyhow::Result<Box<Self>> {
            Ok(Box::new(VoidSemantics {}))
        }
    }

    #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
    enum TestToken {
        Num,
        Plus,
        Minus,
        Mul,
        Div,
        BracketA,
        BracketB,
    }

    impl TokenSet for TestToken {
        fn iter() -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                vec![
                    TestToken::Num,
                    TestToken::Plus,
                    TestToken::Minus,
                    TestToken::Mul,
                    TestToken::Div,
                    TestToken::BracketA,
                    TestToken::BracketB,
                ]
                .into_iter(),
            )
        }

        fn to_regex(token: &Self) -> &'static str {
            match token {
                TestToken::Num => r"^[1-9][0-9]*",
                TestToken::Plus => r"^\+",
                TestToken::Minus => r"^-",
                TestToken::Mul => r"^\*",
                TestToken::Div => r"^/",
                TestToken::BracketA => r"^\(",
                TestToken::BracketB => r"^\)",
            }
        }

        fn ignore_str() -> &'static str {
            r"^[ \t\n]+"
        }
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum TestSyntax {
        ExprPlus,
        ExprMinus,
        Expr2Term,
        TermMul,
        TermDiv,
        Term2Fact,
        Fact2Expr,
        Fact2Num,
    }

    impl pre::Syntax<TestToken, VoidSemantics> for TestSyntax {
        type Parser = LR1<TestToken, TestSyntax, VoidSemantics>;

        fn iter() -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                vec![
                    TestSyntax::ExprPlus,
                    TestSyntax::ExprMinus,
                    TestSyntax::Expr2Term,
                    TestSyntax::TermMul,
                    TestSyntax::TermDiv,
                    TestSyntax::Term2Fact,
                    TestSyntax::Fact2Expr,
                    TestSyntax::Fact2Num,
                ]
                .into_iter(),
            )
        }

        fn to_rule(&self) -> Rule<TestToken> {
            let expr_plus = Rule::from((
                RuleElem::nonterm("expr"),
                vec![
                    RuleElem::nonterm("expr"),
                    RuleElem::term(TestToken::Plus),
                    RuleElem::nonterm("term"),
                ],
            ));

            let expr_minus = Rule::from((
                RuleElem::nonterm("expr"),
                vec![
                    RuleElem::nonterm("expr"),
                    RuleElem::term(TestToken::Minus),
                    RuleElem::nonterm("term"),
                ],
            ));

            let expr_2_term = Rule::<TestToken>::from((
                RuleElem::nonterm("expr"),
                vec![RuleElem::nonterm("term")],
            ));

            let term_mul = Rule::from((
                RuleElem::nonterm("term"),
                vec![
                    RuleElem::nonterm("term"),
                    RuleElem::term(TestToken::Mul),
                    RuleElem::nonterm("fact"),
                ],
            ));

            let term_div = Rule::from((
                RuleElem::nonterm("term"),
                vec![
                    RuleElem::nonterm("term"),
                    RuleElem::term(TestToken::Div),
                    RuleElem::nonterm("fact"),
                ],
            ));

            let term_2_fact = Rule::<TestToken>::from((
                RuleElem::nonterm("term"),
                vec![RuleElem::nonterm("fact")],
            ));

            let fact_2_expr = Rule::from((
                RuleElem::nonterm("fact"),
                vec![
                    RuleElem::term(TestToken::BracketA),
                    RuleElem::nonterm("expr"),
                    RuleElem::term(TestToken::BracketB),
                ],
            ));

            let fact_2_num = Rule::from((RuleElem::nonterm("fact"), vec![]));

            match self {
                TestSyntax::ExprPlus => expr_plus,
                TestSyntax::ExprMinus => expr_minus,
                TestSyntax::Expr2Term => expr_2_term,
                TestSyntax::TermMul => term_mul,
                TestSyntax::TermDiv => term_div,
                TestSyntax::Term2Fact => term_2_fact,
                TestSyntax::Fact2Expr => fact_2_expr,
                TestSyntax::Fact2Num => fact_2_num,
            }
        }
    }

    fn check<T: Into<String>>(
        first_set: &HashMap<&RuleElem<TestToken>, Vec<&RuleElem<TestToken>>>,
        nonterm: T,
        exp_terms: Vec<TestToken>,
    ) {
        let nonterms = RuleElem::<TestToken>::nonterm(nonterm);
        let exp_terms: Vec<RuleElem<TestToken>> = exp_terms
            .into_iter()
            .map(|term| RuleElem::term(term))
            .collect();
        assert!(first_set.get(&nonterms).unwrap().len() == exp_terms.len());

        let result = first_set
            .get(&nonterms)
            .unwrap()
            .into_iter()
            .zip(exp_terms.into_iter())
            .any(|(a, b)| a == &&b);
        assert!(result);
    }

    #[test]
    fn first_set() {
        let ruleset = <TestSyntax as pre::Syntax<TestToken, VoidSemantics>>::syntax();
        let first_set = ruleset.first_set();

        check(
            &first_set,
            "expr",
            vec![
                TestToken::Plus,
                TestToken::Minus,
                TestToken::Mul,
                TestToken::Div,
                TestToken::BracketA,
            ],
        );
        check(
            &first_set,
            "term",
            vec![TestToken::Mul, TestToken::Div, TestToken::BracketA],
        );
        check(&first_set, "fact", vec![TestToken::BracketA]);
    }
}
