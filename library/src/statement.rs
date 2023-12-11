pub mod c;

use std::cell::RefCell;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::constraints::ctime::*;

/// # 集合(文)
///
/// ## 概要
///
/// - Executable な要素の集合を表現します
///
/// ## はめ込み要素
///
/// - stmt (Executable) : 式
///
/// ## 性質
///
/// - Executable
#[derive(Default)]
#[dslblock(namespace = std.statement, property = Executable)]
pub struct StatementSet {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl StatementSet {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { StatementSet -> stmt },
        ];
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}
