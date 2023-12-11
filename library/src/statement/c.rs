use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::constraints::ctime::*;

/// # ブロック
///
/// ## 概要
///
/// - C 言語の ブロック を表現します
///
/// ## はめ込み要素
///
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslblock(namespace = std.statement.c, property = Executable)]
pub struct Block {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl Block {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { Block -> r"\{" stmts r"\}" },
            rule! { Block -> stmt },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
        ];
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

/// # 式-文
///
/// ## 概要
///
/// - C 言語の 式-文を表現します
///
/// ## はめ込み要素
///
/// - expr (Calculatable) : 式として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslblock(namespace = std.statement.c, property = Executable)]
pub struct ExprStatement {
    #[component(single = Calculatable)]
    expr: RefCell<Option<Rule>>,
}

impl ExprStatement {
    fn design(&self) -> Vec<Rule> {
        vec![
            rule! { ExprStatement -> expr ";" },
            self.expr.borrow().clone().unwrap()
        ]
    }
}

/// # if 文
///
/// ## 概要
///
/// - C 言語の if 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslblock(namespace = std.statement.c, property = Executable)]
pub struct If {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl If {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { If -> "if" r"\(" cond r"\)" stmt },
            rule! { If -> "if" r"\(" cond r"\)" stmt else },
            rule! { else -> "else" if },
            rule! { else -> "else" stmt },
        ];
        rules.push(self.cond.borrow().clone().unwrap());
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}
