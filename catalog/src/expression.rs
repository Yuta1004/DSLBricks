pub mod util;

use std::cell::RefCell;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::*;
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
#[derive(Default)]
#[dslbrick(namespace = std.expression, property = Calculatable)]
pub struct ExpressionSet {
    #[component(multiple = Calculatable)]
    expr: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for ExpressionSet {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![rule! { ExpressionSet -> expr }];
        rules.extend(self.expr.borrow().clone());
        rules
    }
}
