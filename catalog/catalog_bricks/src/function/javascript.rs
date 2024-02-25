use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::*;
use crate::constraints::ctime::*;

/// # 関数
///
/// ## 概要
///
/// - JavaScript の関数を表現します
///
/// ## はめ込み要素
///
/// - id (Identifiable) : 引数として使用する識別子を表現する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - DeclaredObject
#[derive(Default)]
#[dslbrick(namespace = std.function.javascript, property = DeclaredObject)]
pub struct Function {
    #[component(single = Identifiable)]
    id: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for Function {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { Function -> "function" id r"\(" args r"\)" r"\{" stmts r"\}" },
            rule! { args -> args "," id },
            rule! { args -> id },
            rule! { args -> },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> },
        ];
        rules.extend(self.id.borrow().clone());
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for Function {
    fn assert(&self) {
        assert!(self.id.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);
    }
}
