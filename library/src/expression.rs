pub mod util;

use std::cell::RefCell;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use crate::common::DSLBlock;
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
#[impl_constraints(Calculatable)]
pub struct ExpressionSet {
    exprs: RefCell<Vec<Rule>>,
}

impl DSLBlock for ExpressionSet {
    fn new() -> Rc<Self> {
        Rc::new(ExpressionSet {
            exprs: RefCell::new(vec![]),
        })
    }
}

impl ExpressionSet {
    pub fn add_expr<T>(self: Rc<Self>, expr: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Calculatable,
    {
        self.exprs
            .borrow_mut()
            .push(rule! { exprs -> [{expr.as_dyn()}] });
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
        self.exprs.borrow().clone().into()
    }
}
