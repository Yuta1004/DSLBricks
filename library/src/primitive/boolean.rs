use std::rc::Rc;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use crate::common::DSLBlock;
use crate::constraints::ctime::*;

/// # 真理値
///
/// ## 概要
///
/// - true または false のいずれかの値を表現します
///
/// ## 性質
///
/// - StaticValue
#[impl_constraints(StaticValue)]
pub struct Boolean;

impl DSLBlock for Boolean {
    fn new() -> Rc<Self> {
        Rc::new(Boolean)
    }
}

impl DSLGeneratable for Boolean {
    fn name(&self) -> &'static str {
        "std.primitive.Boolean"
    }

    fn start(&self) -> &'static str {
        "boolean"
    }

    fn design(&self) -> RuleSet {
        vec![rule! { boolean -> r"(true|false)" }].into()
    }
}
