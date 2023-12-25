use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::*;
use crate::constraints::ctime::*;

/// # 識別子（C言語）
///
/// ## 概要
///
/// - C 言語の定義に従う識別子を表現します
///
/// ## 性質
/// - Identifiable
/// - Calculatable
#[derive(Default)]
#[dslbrick(namespace = std.primitive.identifier, property = Identifiable + Calculatable)]
pub struct CStyleIdentifier;

impl DSLBrickDesign for CStyleIdentifier {
    fn design(&self) -> Vec<Rule> {
        vec![
            rule! { CStyleIdentifier -> "[_a-zA-Z][_a-zA-Z0-9]*" }
        ]
    }
}

impl DSLBrickAssertion for CStyleIdentifier {
    fn assert(&self) {
        // do nothing
    }
}
