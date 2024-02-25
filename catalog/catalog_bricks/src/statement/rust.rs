use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::common::*;
use crate::constraints::ctime::*;

/// # ブロック
///
/// ## 概要
///
/// - Rust のブロックを表現します
///
/// ## はめ込み要素
///
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.rust, property = Executable)]
pub struct Block {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for Block {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { Block -> r"\{" stmts r"\}" },
            rule! { Block -> stmt },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> },
        ];
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for Block {
    fn assert(&self) {
        assert!(self.stmt.borrow().len() > 0)
    }
}

/// # 式-文
///
/// ## 概要
///
/// - Rust の 式-文を表現します
///
/// ## はめ込み要素
///
/// - expr (Calculatable) : 式として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.rust, property = Executable)]
pub struct ExprStatement {
    #[component(single = Calculatable)]
    expr: RefCell<Option<Rule>>,
}

impl DSLBrickDesign for ExprStatement {
    fn design(&self) -> Vec<Rule> {
        vec![
            rule! { ExprStatement -> expr ";" },
            self.expr.borrow().clone().unwrap(),
        ]
    }
}

impl DSLBrickAssertion for ExprStatement {
    fn assert(&self) {
        assert!(self.expr.borrow().is_some());
    }
}

/// # if 文
///
/// ## 概要
///
/// - Rust の if 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.rust, property = Executable)]
pub struct If {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for If {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { If -> "if" cond r"\{" stmts r"\}" },
            rule! { If -> "if" cond r"\{" stmts r"\}" else },
            rule! { else -> "else" if },
            rule! { else -> "else" r"\{" stmts r"\}" },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> },
        ];
        rules.push(self.cond.borrow().clone().unwrap());
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for If {
    fn assert(&self) {
        assert!(self.cond.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);
    }
}

/// # for-in 文
///
/// ## 概要
///
/// - Rust の for-in 文を表現します
///
/// ## はめ込み要素
///
/// - id (Identifiable) : for-in 文内で使用する変数名として使用する構文部品
/// - iter (Calculatable) : for-in 文の実行のために使用するイテレータとしての働きをする構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.rust, property = Executable)]
pub struct ForIn {
    #[component(single = Identifiable)]
    id: RefCell<Option<Rule>>,
    #[component(single = Calculatable)]
    iter: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for ForIn {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { ForIn -> "for" id "in" iter r"\{" stmts r"\}" },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> },
        ];
        rules.push(self.id.borrow().clone().unwrap());
        rules.push(self.iter.borrow().clone().unwrap());
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for ForIn {
    fn assert(&self) {
        assert!(self.id.borrow().is_some());
        assert!(self.iter.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);
    }
}

/// # while 文
///
/// ## 概要
///
/// - Rust の while 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.rust, property = Executable)]
pub struct While {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for While {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { While -> "while" cond r"\{" stmts r"\}" },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> },
        ];
        rules.push(self.cond.borrow().clone().unwrap());
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for While {
    fn assert(&self) {
        assert!(self.cond.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);
    }
}

/// # loop 文
///
/// ## 概要
///
/// - Rust の loop 文を表現します
///
/// ## はめ込み要素
///
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.rust, property = Executable)]
pub struct Loop {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for Loop {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { While -> "loop" r"\{" stmts r"\}" },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> },
        ];
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for Loop {
    fn assert(&self) {
        assert!(self.stmt.borrow().len() > 0);
    }
}
