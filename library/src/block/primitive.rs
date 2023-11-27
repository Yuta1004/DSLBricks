use std::rc::Rc;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

use crate::block::common::DSLBlock;
use crate::block::constraints::ctime::*;

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
        vec![
            rule!{ integer -> r"(-?[1-9][0-9]*|0)" }
        ].into()
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
        vec![
            rule!{ float -> r"(-?[1-9][0-9]*|0)\.[0-9]+" }
        ].into()
    }
}

/// # 文字列
///
/// ## 概要
///
/// - 「"」で囲まれた任意の文字列を表現します
///
/// ## 性質
///
/// - StaticValue
#[impl_constraints(StaticValue)]
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
        vec![
            rule!{ string -> r#"".*""# }
        ].into()
    }
}

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
        vec![
            rule!{ boolean -> r"(true|false)" }
        ].into()
    }
}
