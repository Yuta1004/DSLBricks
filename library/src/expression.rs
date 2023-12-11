pub mod util;

use std::cell::RefCell;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

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
#[derive(Default)]
#[dslblock(namespace = std.expression, property = Calculatable)]
pub struct ExpressionSet {
    #[component(multiple = Calculatable)]
    expr: RefCell<Vec<Rule>>,
}

impl ExpressionSet {
    fn design(&self) -> RuleSet {
        let mut base = vec![
            rule! { ExpressionSet -> expr },
        ];
        base.extend(self.expr.borrow().clone());
        base.into()
    }
}
