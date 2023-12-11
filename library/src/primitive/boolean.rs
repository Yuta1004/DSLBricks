use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use macros::*;

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
#[derive(Default)]
#[dslblock(namespace = std.primitive, property = StaticValue)]
pub struct Boolean;

impl Boolean {
    fn design(&self) -> RuleSet {
        vec![rule! { Boolean -> r"(true|false)" }].into()
    }
}