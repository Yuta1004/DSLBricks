use std::cell::RefCell;

use compiler::bricks::dslbrick;
use compiler::bricks::prelude::*;

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
#[dslbrick(namespace = std.statement.common, property = Executable)]
pub struct StatementSet {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for StatementSet {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![rule! { StatementSet -> stmt }];
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for StatementSet {
    fn assert(&self) {
        assert!(self.stmt.borrow().len() > 0);
    }
}
