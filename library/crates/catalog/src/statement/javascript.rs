use std::cell::RefCell;
use std::rc::Rc;

use compiler::bricks::dslbrick;
use compiler::bricks::prelude::*;

use crate::constraints::ctime::*;

/// # ブロック
///
/// ## 概要
///
/// - JavsScript の ブロック を表現します
///
/// ## はめ込み要素
///
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.javascript, property = Executable)]
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
/// - JavaScript の 式-文を表現します
///
/// ## はめ込み要素
///
/// - expr (Calculatable) : 式として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.javascript, property = Executable)]
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
/// - JavaScript の if 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.javascript, property = Executable)]
pub struct If {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for If {
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

impl DSLBrickAssertion for If {
    fn assert(&self) {
        assert!(self.cond.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);
    }
}

/// # for 文
///
/// ## 概要
///
/// - JavaScript の for 文を表現します
///
/// ## はめ込み要素
///
/// - init (Executable) : 初期化文として使用する構文部品
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - incr (Calculatable) : 増減式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.javascript, property = Executable)]
pub struct For {
    #[component(single = Calculatable)]
    init: RefCell<Option<Rule>>,
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(single = Calculatable)]
    incr: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for For {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { For -> "for" r"\(" init_or_null ";" cond_or_null ";" incr_or_null r"\)" stmt },
            rule! { init_or_null -> init },
            rule! { init_or_null -> },
            rule! { cond_or_null -> cond },
            rule! { cond_or_null -> },
            rule! { incr_or_null -> incr },
            rule! { incr_or_null -> },
        ];
        rules.push(self.init.borrow().clone().unwrap());
        rules.push(self.cond.borrow().clone().unwrap());
        rules.push(self.incr.borrow().clone().unwrap());
        rules.extend(self.stmt.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for For {
    fn assert(&self) {
        assert!(self.init.borrow().is_some());
        assert!(self.cond.borrow().is_some());
        assert!(self.incr.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);
    }
}

/// # while 文
///
/// ## 概要
///
/// - JavaScript の while 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.javascript, property = Executable)]
pub struct While {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for While {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![rule! { While -> "while" r"\(" cond r"\)" stmt }];
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
