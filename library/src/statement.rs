pub mod c;

use std::cell::RefCell;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::DSLBlockBuilder;

use crate::common::DSLBlock;
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
#[derive(DSLBlockBuilder)]
#[impl_constraints(Executable)]
pub struct StatementSet {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBlock for StatementSet {
    fn new() -> Rc<Self> {
        Rc::new(StatementSet {
            stmt: RefCell::new(vec![]),
        })
    }
}

impl DSLGeneratable for StatementSet {
    fn name(&self) -> &'static str {
        "std.statement.StatementSet"
    }

    fn start(&self) -> &'static str {
        "stmts"
    }

    fn design(&self) -> RuleSet {
        self.stmt.borrow().clone().into()
    }
}
