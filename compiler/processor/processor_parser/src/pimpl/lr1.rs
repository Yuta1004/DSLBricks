use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

use itertools::Itertools;

use lexer::{Token, LexIterator};

use super::super::rule::{Rule, RuleElem, RuleSet};
use super::super::syntax::{ASyntax, Syntax};
use super::super::ParseError;
use super::ParserImpl;

#[derive(Debug)]
enum LRAction<S> {
    Shift(usize),
    Reduce(S, usize, usize), // syntax, goto_id, elems_cnt
    Accept,
    None,
}

pub struct LR1<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    semantics: PhantomData<A>,
    action_table: Vec<HashMap<T, LRAction<S>>>,
    eof_action_table: Vec<LRAction<S>>,
    goto_table: Vec<Vec<usize>>,
}

impl<A, S, T> ParserImpl<A, S, T> for LR1<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token + 'static,
{
    fn setup() -> anyhow::Result<Self> {
        // 1. Pre-process
        let ruleset = S::syntax();
        let first_set = ruleset.first_set();

        // 2. Generate dummy nonterm
        let top_dummy: Rule<T> = Rule::from((
            RuleElem::nonterm("__top_dummy"),
            vec![RuleElem::nonterm(&ruleset.top)],
        ));
        let top_dummy = vec![LRItem::new(
            &top_dummy,
            HashSet::from_iter(vec![&RuleElem::EOF]),
        )];
        let lr_items = LRItemSet::new(0, HashSet::from_iter(top_dummy));
        let lr_items = lr_items.expand_closure(&ruleset, &first_set);

        // 3. Generate a DFA
        let dfa = LRItemDFA::gen(lr_items, &ruleset, &first_set);

        // 4. Initialize tables
        let mut idx = 0;
        let mut nonterm_table = HashMap::new();
        for relem in ruleset.nonterms() {
            if let RuleElem::NonTerm(s) = &relem {
                if !nonterm_table.contains_key(s) {
                    nonterm_table.insert(s.to_string(), idx);
                    idx += 1;
                }
            }
        }

        let rule_table: Vec<S> = S::iter().collect();

        let mut action_table: Vec<HashMap<T, LRAction<S>>> = Vec::with_capacity(dfa.0.len());
        let mut eof_action_table: Vec<LRAction<S>> = Vec::with_capacity(dfa.0.len());
        let mut goto_table: Vec<Vec<usize>> = Vec::with_capacity(dfa.0.len());
        for _ in 0..dfa.0.len() {
            action_table.push(HashMap::from_iter(
                T::iter()
                    .map(|token| (token, LRAction::None))
                    .collect::<Vec<(T, LRAction<S>)>>(),
            ));
            eof_action_table.push(LRAction::None);
            goto_table.push(vec![0; nonterm_table.keys().len()]);
        }

        // 5. Setup tables
        for lritem_set in &dfa.0 {
            for (token, next) in &lritem_set.next {
                match &token {
                    RuleElem::NonTerm(s) => {
                        let id = lritem_set.id as usize;
                        let label = *nonterm_table.get(s).unwrap();
                        goto_table[id][label] = *next as usize;
                    }
                    RuleElem::Term(t) => {
                        let id = lritem_set.id as usize;
                        let label = action_table[id].get_mut(t).unwrap();
                        *label = LRAction::Shift(*next as usize);
                    }
                    _ => {}
                }
            }

            for item in &lritem_set.lr_items {
                if item.dot_pos != item.rule.right.len() {
                    continue;
                }
                if let RuleElem::NonTerm(left) = &item.rule.left {
                    for la_token in &item.la_tokens {
                        if let RuleElem::Term(t) = la_token {
                            let id = lritem_set.id as usize;
                            let label = action_table[id].get_mut(t).unwrap();
                            *label = LRAction::Reduce(
                                rule_table[item.rule.id as usize],
                                *nonterm_table.get(left).unwrap(),
                                item.rule.right.len(),
                            );
                        }
                        if let RuleElem::EOF = la_token {
                            let id = lritem_set.id as usize;
                            eof_action_table[id] = if left == "__top_dummy" {
                                LRAction::Accept
                            } else {
                                LRAction::Reduce(
                                    rule_table[item.rule.id as usize],
                                    *nonterm_table.get(left).unwrap(),
                                    item.rule.right.len(),
                                )
                            };
                        }
                    }
                }
            }
        }

        Ok(LR1 {
            semantics: PhantomData,
            action_table,
            eof_action_table,
            goto_table,
        })
    }

    fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<A>> {
        let mut stack = vec![0];
        let mut result = vec![];
        loop {
            let input = lexer.next();
            loop {
                let top = stack[stack.len() - 1];
                let action = match input {
                    Some((s, token)) => (self.action_table[top].get(&token).unwrap(), Some(s)),
                    None => (&self.eof_action_table[top], None),
                };
                match action.0 {
                    LRAction::Shift(new_state) => {
                        stack.push(*new_state);
                        result.push((None, Some(action.1.unwrap())));
                        break;
                    }
                    LRAction::Reduce(syntax, goto, elems_cnt) => {
                        stack.truncate(stack.len() - elems_cnt);
                        stack.push(self.goto_table[stack[stack.len() - 1]][*goto]);

                        let mut result0 = vec![];
                        for _ in 0..*elems_cnt {
                            result0.push(result.pop().unwrap());
                        }
                        result0.reverse();
                        result.push((Some(A::mapping(*syntax, result0)?), None));
                    }
                    LRAction::None => {
                        let remain = match lexer.remain() {
                            Some(remain) => remain.to_string(),
                            None => "".to_string(),
                        };
                        return Err(ParseError::from(remain).into());
                    }
                    LRAction::Accept => return Ok(result.pop().unwrap().0.unwrap()),
                }
            }
        }
    }
}

#[derive(Debug)]
struct LRItemDFA<'a, T: Token>(Vec<LRItemSet<'a, T>>);

impl<'a, T: Token> LRItemDFA<'a, T> {
    fn gen(
        init_set: LRItemSet<'a, T>,
        ruleset: &'a RuleSet<T>,
        first_set: &HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> LRItemDFA<'a, T> {
        let issue_id = |old_sets: &Vec<LRItemSet<'a, T>>, set: &LRItemSet<'a, T>| {
            if let Some(ex_set) = old_sets.iter().find(|&set0| set0.strict_eq(set)) {
                Err(ex_set.id)
            } else {
                Ok(old_sets.len() as i32)
            }
        };

        // "Expand a closure" <--> "Generate next nodes" loop
        let mut loop_idx = (0, 1);
        let mut lritem_sets = vec![init_set];
        while loop_idx.0 != loop_idx.1 {
            let mut new_found_cnt = 0;
            for idx in loop_idx.0..loop_idx.1 {
                let next_sets = lritem_sets[idx].gen_next_sets(ruleset, first_set);
                for (bef_token, mut next_set) in next_sets {
                    match issue_id(&lritem_sets, &next_set) {
                        Ok(id) => {
                            next_set.id = id;
                            lritem_sets[idx].next.insert(bef_token, id);
                            lritem_sets.push(next_set);
                            new_found_cnt += 1;
                        }
                        Err(id) => {
                            lritem_sets[idx].next.insert(bef_token, id);
                        }
                    }
                }
            }
            loop_idx = (loop_idx.1, loop_idx.1 + new_found_cnt);
        }

        LRItemDFA(lritem_sets)
    }
}

#[derive(Clone, Debug)]
struct LRItemSet<'a, T: Token> {
    id: i32,
    next: HashMap<&'a RuleElem<T>, i32>,
    lr_items: HashSet<LRItem<'a, T>>,
}

impl<'a, T: Token> PartialEq for LRItemSet<'a, T> {
    fn eq(&self, other: &LRItemSet<'a, T>) -> bool {
        self.lr_items == other.lr_items
    }
}

impl<'a, T: Token> PartialEq<HashSet<LRItem<'a, T>>> for LRItemSet<'a, T> {
    fn eq(&self, other: &HashSet<LRItem<'a, T>>) -> bool {
        &self.lr_items == other
    }
}

impl<'a, T: Token> Eq for LRItemSet<'a, T> {}

impl<'a, T: Token> LRItemSet<'a, T> {
    fn new(id: i32, lr_items: HashSet<LRItem<'a, T>>) -> Self {
        LRItemSet {
            id,
            next: HashMap::new(),
            lr_items,
        }
    }

    fn strict_eq(&self, other: &Self) -> bool {
        if self.lr_items.len() != other.lr_items.len() {
            return false;
        }
        self.lr_items
            .iter()
            .all(|item| other.lr_items.iter().any(|item_b| item_b.strict_eq(item)))
    }

    fn expand_closure<'b>(
        mut self,
        ruleset: &'a RuleSet<T>,
        first_set: &'b HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> LRItemSet<'a, T> {
        let mut lr_items = self.lr_items.clone();
        let mut lr_items_fetched = self.lr_items;
        loop {
            let new_items: Vec<LRItem<'_, T>> = lr_items_fetched
                .iter()
                .flat_map(|item| item.expand_closure(ruleset, first_set))
                .collect();
            let new_items = LRItem::<'a, T>::unify_all(new_items);
            let new_items = HashSet::from_iter(new_items);

            let bef_len = lr_items.len();
            lr_items = LRItem::<'a, T>::unity_set(lr_items, new_items.clone());
            let af_len = lr_items.len();
            if bef_len == af_len {
                break;
            }
            lr_items_fetched = new_items;
        }
        self.lr_items = lr_items;

        self
    }

    fn gen_next_sets<'b>(
        &self,
        ruleset: &'a RuleSet<T>,
        first_set: &'b HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> HashMap<&'a RuleElem<T>, LRItemSet<'a, T>> {
        let new_items: Vec<(&'a RuleElem<T>, LRItem<'a, T>)> = self
            .lr_items
            .iter()
            .filter_map(|lr_item| lr_item.next_dot())
            .collect();

        let mut new_sets: HashMap<&RuleElem<T>, HashSet<LRItem<'a, T>>> = HashMap::new();
        for (bef_token, lr_item) in new_items {
            if new_sets.get(&bef_token).is_none() {
                new_sets.insert(bef_token, HashSet::new());
            }
            new_sets.get_mut(&bef_token).unwrap().insert(lr_item);
        }

        let mut new_sets_expanded: HashMap<&'a RuleElem<T>, LRItemSet<'a, T>> = HashMap::new();
        for (ktoken, new_set) in new_sets {
            let new_set = LRItemSet::new(0, new_set);
            let new_set = new_set.expand_closure(ruleset, first_set);
            new_sets_expanded.insert(ktoken, new_set);
        }

        new_sets_expanded
    }
}

#[derive(Clone, Debug)]
struct LRItem<'a, T: Token> {
    rule: &'a Rule<T>,
    dot_pos: usize,
    la_tokens: HashSet<&'a RuleElem<T>>,
}

impl<'a, T: Token> Hash for LRItem<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.dot_pos.hash(state);
    }
}

impl<'a, T: Token> PartialEq for LRItem<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.rule == other.rule && self.dot_pos == other.dot_pos
    }
}

impl<'a, T: Token> Eq for LRItem<'a, T> {}

impl<'a, T: Token> LRItem<'a, T> {
    fn new(rule: &'a Rule<T>, la_tokens: HashSet<&'a RuleElem<T>>) -> LRItem<'a, T> {
        LRItem {
            rule,
            dot_pos: 0,
            la_tokens,
        }
    }

    fn strict_eq(&self, other: &Self) -> bool {
        self.rule == other.rule
            && self.dot_pos == other.dot_pos
            && self.la_tokens == other.la_tokens
    }

    fn expand_closure<'b>(
        &self,
        ruleset: &'a RuleSet<T>,
        first_set: &'b HashMap<&'a RuleElem<T>, Vec<&'a RuleElem<T>>>,
    ) -> HashSet<LRItem<'a, T>> {
        let af_la_tokens = if self.dot_pos + 1 < self.rule.right.len() {
            HashSet::from_iter(
                first_set
                    .get(&self.rule.right[self.dot_pos + 1])
                    .unwrap()
                    .clone(),
            )
        } else {
            self.la_tokens.clone()
        };

        if self.dot_pos < self.rule.right.len()
            && matches!(self.rule.right[self.dot_pos], RuleElem::NonTerm(_))
        {
            ruleset
                .find_rule(&self.rule.right[self.dot_pos])
                .into_iter()
                .map(|rule| LRItem::<'a, T>::new(rule, af_la_tokens.clone()))
                .collect()
        } else {
            HashSet::new()
        }
    }

    #[allow(clippy::int_plus_one)]
    fn next_dot(&self) -> Option<(&'a RuleElem<T>, LRItem<'a, T>)> {
        if self.dot_pos + 1 <= self.rule.right.len() {
            let bef_token = &self.rule.right[self.dot_pos];
            let item = LRItem {
                rule: self.rule,
                dot_pos: self.dot_pos + 1,
                la_tokens: self.la_tokens.clone(),
            };
            Some((bef_token, item))
        } else {
            None
        }
    }

    fn unify(&mut self, other: LRItem<'a, T>) {
        if self != &other {
            return;
        }
        other.la_tokens.into_iter().for_each(|la_token| {
            if !self.la_tokens.contains(&la_token) {
                self.la_tokens.insert(la_token);
            }
        });
    }

    fn unify_all(mut items: Vec<LRItem<'a, T>>) -> Vec<LRItem<'a, T>> {
        for idx in (0..items.len()).permutations(2) {
            let (a_idx, b_idx) = (idx[0], idx[1]);
            let tmp = items[b_idx].clone();
            items[a_idx].unify(tmp);
        }
        items
    }

    fn unity_set(
        items_a: HashSet<LRItem<'a, T>>,
        items_b: HashSet<LRItem<'a, T>>,
    ) -> HashSet<LRItem<'a, T>> {
        let mut items_a = Vec::from_iter(items_a);
        let items_b = Vec::from_iter(items_b);
        items_a.extend(items_b);
        HashSet::from_iter(Self::unify_all(items_a))
    }
}

#[cfg(test)]
mod test {
    use strum::EnumIter;

    use lexer::{Lexer, Token};

    use crate::rule::{Rule, RuleElem};
    use crate::syntax::{ASyntax, Syntax};
    use crate::{Parser, LR1};

    pub struct VoidSemantics;

    impl<S, T> ASyntax<S, T> for VoidSemantics
    where
        S: Syntax<Self, T>,
        T: Token,
    {
        fn mapping(_: S, _: Vec<(Option<Box<Self>>, Option<&str>)>) -> anyhow::Result<Box<Self>> {
            Ok(Box::new(VoidSemantics {}))
        }
    }

    #[derive(EnumIter, Clone, Copy, Hash, PartialEq, Eq, Debug)]
    enum TestToken {
        Num,
        Plus,
        Minus,
        Mul,
        Div,
        BracketA,
        BracketB,
    }

    impl Token for TestToken {
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

    #[derive(EnumIter, Clone, Copy, Debug)]
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

    impl Syntax<VoidSemantics, TestToken> for TestSyntax {
        type Parser = LR1<VoidSemantics, TestSyntax, TestToken>;

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

            let fact_2_num = Rule::from((
                RuleElem::nonterm("fact"),
                vec![RuleElem::term(TestToken::Num)],
            ));

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

    #[test]
    fn input_ok() {
        let inputs = vec![
            "10",
            "10 + 20",
            "10 - 20",
            "10 * 20",
            "10 / 20",
            "10 + 20 * 30 - 40",
            "(10)",
            "((((10))))",
            "10 * (20 - 30)",
            "((10 + 20) * (30 / 40)) - 50",
        ];
        for input in inputs {
            assert!(
                parse::<VoidSemantics, TestSyntax, TestToken>(input),
                "{}",
                input
            )
        }
    }

    #[test]
    fn input_err() {
        let inputs = vec![
            "()",
            "(10 -",
            "10 +",
            "*",
            "10 20 + 30",
            "10 + 20 * 30 / 40 (",
            "(((10))",
        ];
        for input in inputs {
            assert!(
                !parse::<VoidSemantics, TestSyntax, TestToken>(input),
                "{}",
                input
            )
        }
    }

    fn parse<A, S, T>(input: &str) -> bool
    where
        A: ASyntax<S, T>,
        S: Syntax<A, T>,
        T: Token + 'static,
    {
        let lexer = Lexer::<T>::new().unwrap();
        let parser = Parser::<A, S, T>::new().unwrap();

        parser.parse(&mut lexer.lex(input)).is_ok()
    }
}
