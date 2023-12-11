use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::DSLBlockBuilder;

use crate::common::DSLBlock;
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
#[derive(DSLBlockBuilder)]
#[impl_constraints(Executable)]
pub struct Block {
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBlock for Block {
    fn new() -> Rc<Self> {
        Rc::new(Block {
            stmt: RefCell::new(vec![]),
        })
    }
}

impl DSLGeneratable for Block {
    fn name(&self) -> &'static str {
        "std.statement.c.Block"
    }

    fn start(&self) -> &'static str {
        "block"
    }

    fn design(&self) -> RuleSet {
        assert!(self.stmt.borrow().len() > 0);

        let mut base = vec![
            rule! { block -> r"\{" stmts r"\}" },
            rule! { block -> stmt },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
        ];
        base.extend(self.stmt.borrow().clone());

        base.into()
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
#[derive(DSLBlockBuilder)]
#[impl_constraints(Executable)]
pub struct ExprStatement {
    #[component(single = Calculatable)]
    expr: RefCell<Option<Rule>>,
}

impl DSLBlock for ExprStatement {
    fn new() -> Rc<Self> {
        Rc::new(ExprStatement {
            expr: RefCell::new(None)
        })
    }
}

impl DSLGeneratable for ExprStatement {
    fn name(&self) -> &'static str {
        "std.statement.c.ExprStatement"
    }

    fn start(&self) -> &'static str {
        "stmt"
    }

    fn design(&self) -> RuleSet {
        assert!(self.expr.borrow().is_some());

        vec![
            rule! { stmt -> expr ";" },
            self.expr.borrow().clone().unwrap()
        ].into()
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
#[derive(DSLBlockBuilder)]
#[impl_constraints(Executable)]
pub struct If {
    #[component(single = Calculatable)]
    cond: RefCell<Option<Rule>>,
    #[component(multiple = Executable)]
    stmt: RefCell<Vec<Rule>>,
}

impl DSLBlock for If {
    fn new() -> Rc<Self> {
        Rc::new(If {
            cond: RefCell::new(None),
            stmt: RefCell::new(vec![]),
        })
    }
}

impl DSLGeneratable for If {
    fn name(&self) -> &'static str {
        "std.statement.c.If"
    }

    fn start(&self) -> &'static str {
        "if"
    }

    fn design(&self) -> RuleSet {
        assert!(self.cond.borrow().is_some());
        assert!(self.stmt.borrow().len() > 0);

        let mut base = vec![
            rule! { if -> "if" r"\(" cond r"\)" stmt },
            rule! { if -> "if" r"\(" cond r"\)" stmt else },
            rule! { else -> "else" if },
            rule! { else -> "else" stmt },
        ];
        base.push(self.cond.borrow().clone().unwrap());
        base.extend(self.stmt.borrow().clone());

        base.into()
    }
}
