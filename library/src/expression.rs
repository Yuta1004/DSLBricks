pub mod util;

use std::cell::RefCell;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::DSLBlockBuilder;

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
#[derive(DSLBlockBuilder)]
#[impl_constraints(Calculatable)]
pub struct ExpressionSet {
    #[component(multiple = Calculatable)]
    expr: RefCell<Vec<Rule>>,
}

impl DSLBlock for ExpressionSet {
    fn new() -> Rc<Self> {
        Rc::new(ExpressionSet {
            expr: RefCell::new(vec![]),
        })
    }
}

impl DSLGeneratable for ExpressionSet {
    fn name(&self) -> &'static str {
        "std.expression.ExpressionSet"
    }

    fn start(&self) -> &'static str {
        "expr"
    }

    fn design(&self) -> RuleSet {
        self.expr.borrow().clone().into()
    }
}
