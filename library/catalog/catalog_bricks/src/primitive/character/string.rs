use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::*;

/// # 文字列
///
/// ## 概要
///
/// - 「"」で囲まれた任意の文字列を表現します
///
/// ## 性質
///
/// - StaticValue
#[derive(Default)]
#[dslbrick(namespace = std.primitive.character.string)]
pub struct String;

impl DSLBrickDesign for String {
    fn design(&self) -> Vec<Rule> {
        vec![rule! { String -> r#"".*""# }]
    }
}

impl DSLBrickAssertion for String {
    fn assert(&self) {
        // do nothing
    }
}
