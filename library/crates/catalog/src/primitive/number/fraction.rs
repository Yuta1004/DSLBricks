// TODO: 2進表現
// TODO: 8進表現
// TODO: 16進表現

use compiler::bricks::dslbrick;
use compiler::bricks::prelude::*;

use crate::constraints::ctime::*;

/// # 10進数 小数
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
#[dslbrick(namespace = std.primitive.number.fraction, property = Calculatable)]
pub struct DecimalFraction;

impl DSLBrickDesign for DecimalFraction {
    fn design(&self) -> Vec<Rule> {
        vec![rule! { DecimalFraction -> r"(-?[1-9][0-9]*|0)\.[0-9]+" }]
    }
}

impl DSLBrickAssertion for DecimalFraction {
    fn assert(&self) {
        // do nothing
    }
}
