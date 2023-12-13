use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use macros::*;

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
#[derive(Default)]
#[dslbrick(namespace = std.primitive, property = StaticValue + Calculatable)]
pub struct Integer;

impl Integer {
    fn design(&self) -> Vec<Rule> {
        vec![rule! { Integer -> r"(-?[1-9][0-9]*|0)" }]
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
#[derive(Default)]
#[dslbrick(namespace = std.primitive, property = StaticValue + Calculatable)]
pub struct Float;

impl Float {
    fn design(&self) -> Vec<Rule> {
        vec![rule! { Float -> r"(-?[1-9][0-9]*|0)\.[0-9]+" }]
    }
}
