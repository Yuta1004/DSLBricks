use std::rc::Rc;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::DSLBlock;
use crate::constraints::ctime::*;

/// # 文字列
///
/// ## 概要
///
/// - 「"」で囲まれた任意の文字列を表現します
///
/// ## 性質
///
/// - StaticValue
#[dslblock(StaticValue)]
pub struct String;

impl DSLBlock for String {
    fn new() -> Rc<Self> {
        Rc::new(String)
    }
}

impl DSLGeneratable for String {
    fn name(&self) -> &'static str {
        "std.primitive.String"
    }

    fn start(&self) -> &'static str {
        "string"
    }

    fn design(&self) -> RuleSet {
        vec![rule! { string -> r#"".*""# }].into()
    }
}
