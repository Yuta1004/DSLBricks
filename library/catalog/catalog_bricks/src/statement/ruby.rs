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
/// - Ruby の ブロック を表現します
///
/// ## はめ込み要素
///
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.ruby, property = Executable)]
pub struct Block {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for Block {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { Block -> r"\{" stmts r"\}" },
            rule! { Block -> "do" stmts "end" },
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

/// # if 文
///
/// ## 概要
///
/// - Ruby の if 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.ruby, property = Executable)]
pub struct If {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for If {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { If -> "if" cond "then" stmts elsif },
            rule! { If -> "if" cond "then" stmts else },
            rule! { If -> "if" cond "then" stmts "end" },
            rule! { elsif -> "elsif" cond "then" stmts elsif },
            rule! { elsif -> "elsif" cond "then" stmts else },
            rule! { elsif -> "elsif" cond "then" stmts "end" },
            rule! { else -> "else" stmts "end" },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
            rule! { stmts -> }
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
/// - Ruby の for 文を表現します
///
/// ## はめ込み要素
///
/// - id (Identifiable) : for 文内で使用する変数名として使用する構文部品
/// - iter (Calculatable) : for 文の実行のために使用するイテレータとしての働きをする構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.ruby, property = Executable)]
pub struct For {
    #[component(single = Identifiable)]
    id: RefCell<Option<Rule>>,
    #[component(single = Calculatable)]
    iter: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for For {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { ForIn -> "for" id "in" iter "do" stmts "end" },
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

impl DSLBrickAssertion for For {
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
/// - Ruby の while 文を表現します
///
/// ## はめ込み要素
///
/// - cond (Calculatable) : 条件式として使用する構文部品
/// - stmt (Executable) : 実行される文として使用する構文部品
///
/// ## 性質
/// - Executable
#[derive(Default)]
#[dslbrick(namespace = std.statement.ruby, property = Executable)]
pub struct While {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBrickDesign for While {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { While -> "while" cond "do" stmts "end" },
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
