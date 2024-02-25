// TODO: 2進表現
// TODO: 8進表現
// TODO: 16進表現

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::*;
use crate::constraints::ctime::*;

/// # 10進数 整数
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
#[dslbrick(namespace = std.primitive.number.integer, property = Calculatable)]
pub struct DecimalInteger;

impl DSLBrickDesign for DecimalInteger {
    fn design(&self) -> Vec<Rule> {
        vec![rule! { DecimalInteger -> r"(-?[1-9][0-9]*|0)" }]
    }
}

impl DSLBrickAssertion for DecimalInteger {
    fn assert(&self) {
        // do nothing
    }
}
