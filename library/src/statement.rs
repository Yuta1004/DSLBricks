pub mod c;

use std::cell::RefCell;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

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
#[impl_constraints(Executable)]
pub struct StatementSet {
    stmts: RefCell<Vec<Rule>>,
}

impl DSLBlock for StatementSet {
    fn new() -> Rc<Self> {
        Rc::new(StatementSet {
            stmts: RefCell::new(vec![]),
        })
    }
}

impl StatementSet {
    pub fn add_stmt<T>(self: Rc<Self>, stmt: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Executable,
    {
        self.stmts.borrow_mut().push(rule! { stmts -> [{stmt.as_dyn()}] });
        self
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
        self.stmts.borrow().clone().into()
    }
}
