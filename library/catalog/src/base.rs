use std::cell::RefCell;
use std::rc::Rc;

use compiler::bricks::dslbrick;
use compiler::bricks::prelude::*;

use crate::constraints::ctime::*;

/// # 宣言を基本単位とする言語構造
///
/// ## 概要
///
/// - 関数や構造体のように，宣言記述をトップレベルにもつ言語を表現します
/// - 構文部品を 1 つにまとめるために使用されることを想定しています
///
/// ## はめ込み要素
///
/// - declare (DeclaredObject) : 宣言記述を扱う構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.base, property = Executable)]
pub struct DeclaringBaseLanguage {
    #[component(multiple = DeclaredObject)]
    declare: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for DeclaringBaseLanguage {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { DeclaringBaseLanguage -> declare_list },
            rule! { declare_list -> declare_list declare },
            rule! { declare_list -> declare },
            rule! { declare_list -> },
        ];
        rules.extend(self.declare.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for DeclaringBaseLanguage {
    fn assert(&self) {
        assert!(self.declare.borrow().len() > 0)
    }
}

/// # 式を基本単位とする言語構造
///
/// ## 概要
///
/// - 算術式や if 式のように，式をトップレベルにもつ言語を表現します
/// - 構文部品を 1 つにまとめるために使用されることを想定しています
///
/// ## はめ込み要素
///
/// - expr (Calculatable) : 式を扱う構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.base, property = Executable)]
pub struct ExpressionBaseLanguage {
    #[component(multiple = Calculatable)]
    expr: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for ExpressionBaseLanguage {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { DeclaringBaseLanguage -> exprs },
            rule! { exprs -> exprs expr },
            rule! { exprs -> expr },
            rule! { exprs -> },
        ];
        rules.extend(self.expr.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for ExpressionBaseLanguage {
    fn assert(&self) {
        assert!(self.expr.borrow().len() > 0)
    }
}
