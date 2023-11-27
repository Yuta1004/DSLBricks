pub mod c;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use crate::block::constraints::ctime::*;

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
    stmts: Vec<Rule>,
}

impl StatementSet {
    pub fn new() -> StatementSet {
        StatementSet {
            stmts: vec![],
        }
    }

    pub fn add_stmt<T>(mut self, stmt: T) -> StatementSet
    where
        T: Executable + 'static,
    {
        self.stmts.push(rule! { stmts -> [stmt] });
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
        self.stmts.clone().into()
    }
}
