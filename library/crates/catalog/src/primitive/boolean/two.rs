use compiler::bricks::dslbrick;
use compiler::bricks::prelude::*;

/// # 2-状態 真理値
///
/// ## 概要
///
/// - true または false のいずれかの値を表現します
///
/// ## 性質
///
/// - StaticValue
#[derive(Default)]
#[dslbrick(namespace = std.primitive.boolean.two)]
pub struct Boolean;

impl DSLBrickDesign for Boolean {
    fn design(&self) -> Vec<Rule> {
        vec![rule! { Boolean -> r"(true|false)" }]
    }
}

impl DSLBrickAssertion for Boolean {
    fn assert(&self) {
        // do nothing
    }
}
