use std::rc::Rc;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use crate::common::DSLBlock;
use crate::constraints::ctime::*;

/// # 整数
///
/// ## 概要
///
/// - 正あるいは負の整数，および 0 を表現します
///
/// ## 性質
///
/// - StaticValue
/// - Calculatable
#[impl_constraints(StaticValue, Calculatable)]
pub struct Integer;

impl DSLBlock for Integer {
    fn new() -> Rc<Integer> {
        Rc::new(Integer)
    }
}

impl DSLGeneratable for Integer {
    fn name(&self) -> &'static str {
        "std.primitive.Integer"
    }

    fn start(&self) -> &'static str {
        "integer"
    }

    fn design(&self) -> RuleSet {
        vec![rule! { integer -> r"(-?[1-9][0-9]*|0)" }].into()
    }
}

/// # 小数
///
/// ## 概要
///
/// - 正あるいは負の小数，および 0.0..0 を表現します
///
/// ## 性質
///
/// - StaticValue
/// - Calculatable
#[impl_constraints(StaticValue, Calculatable)]
pub struct Float;

impl DSLBlock for Float {
    fn new() -> Rc<Self> {
        Rc::new(Float)
    }
}

impl DSLGeneratable for Float {
    fn name(&self) -> &'static str {
        "std.primitive.Float"
    }

    fn start(&self) -> &'static str {
        "float"
    }

    fn design(&self) -> RuleSet {
        vec![rule! { float -> r"(-?[1-9][0-9]*|0)\.[0-9]+" }].into()
    }
}
