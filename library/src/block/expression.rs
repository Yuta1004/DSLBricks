pub mod util;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use crate::constraints::ctime::*;

/// # 集合(式)
///
/// ## 概要
///
/// - Calculatable な要素の集合を表現します
///
/// ## はめ込み要素
///
/// - expr (Calculatable) : 式
///
/// ## 性質
///
/// - Calculatable
#[derive(Clone)]
#[impl_constraints(Calculatable)]
pub struct ExpressionSet {
    exprs: Vec<Rule>,
}

impl ExpressionSet {
    pub fn new() -> ExpressionSet {
        ExpressionSet {
            exprs: vec![],
        }
    }

    pub fn add_expr<T>(mut self, expr: T) -> ExpressionSet
    where
        T: Calculatable + 'static,
    {
        self.exprs.push(rule! { exprs -> [expr] });
        self
    }
}

impl DSLGeneratable for ExpressionSet {
    fn name(&self) -> &'static str {
        "std.expression.ExpressionSet"
    }

    fn start(&self) -> &'static str {
        "exprs"
    }

    fn design(&self) -> RuleSet {
        self.exprs.clone().into()
    }
}
