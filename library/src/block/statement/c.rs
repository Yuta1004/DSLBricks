use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use crate::block::common::DSLBlock;
use crate::block::constraints::ctime::*;

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
#[impl_constraints(Executable)]
pub struct Block {
    stmts: RefCell<Vec<Rule>>,
}

impl DSLBlock for Block {
    fn new() -> Rc<Self> {
        Rc::new(Block {
            stmts: RefCell::new(vec![]),
        })
    }
}

impl Block {
    pub fn add_stmt<T>(self: Rc<Self>, stmt: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Executable + 'static,
    {
        self.stmts.borrow_mut().push(rule! { stmt -> [{stmt.into()}] });
        self
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
        assert!(self.stmts.borrow().len() > 0);

        let mut base = vec![
            rule! { block -> r"\{" stmts r"\}" },
            rule! { block -> stmt },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
        ];
        base.extend(self.stmts.borrow().clone());

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
#[impl_constraints(Executable)]
pub struct ExprStatement {
    expr: Option<Rule>,
}

impl DSLBlock for ExprStatement {
    fn new() -> Rc<Self> {
        Rc::new(ExprStatement {
            expr: None,
        })
    }
}

impl ExprStatement {
    pub fn set_expr<T>(self: Rc<Self>, expr: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Calculatable + 'static,
    {
        Rc::new(ExprStatement {
            expr: Some(rule! { stmt -> [{expr.into()}] ";" })
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
        assert!(self.expr.is_some());
        vec![self.expr.clone().unwrap()].into()
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
#[impl_constraints(Executable)]
pub struct If {
    cond: Option<Rule>,
    stmts: RefCell<Vec<Rule>>,
}

impl DSLBlock for If {
    fn new() -> Rc<Self> {
        Rc::new(If {
            cond: None,
            stmts: RefCell::new(vec![]),
        })
    }
}

impl If {
    pub fn set_cond<T>(self: Rc<Self>, cond: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Calculatable + 'static,
    {
        Rc::new(If {
            cond: Some(rule! { cond -> [{cond.into()}] }),
            stmts: RefCell::clone(&self.stmts),
        })
    }

    pub fn add_stmt<T>(self: Rc<Self>, stmt: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Executable + 'static,
    {
        self.stmts.borrow_mut().push(rule! { stmt -> [{stmt.into()}] });
        self
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
        assert!(self.cond.is_some());
        assert!(self.stmts.borrow().len() > 0);

        let mut base = vec![
            rule! { if -> "if" r"\(" cond r"\)" stmt },
            rule! { if -> "if" r"\(" cond r"\)" stmt else },
            rule! { else -> "else" if },
            rule! { else -> "else" stmt },
        ];
        base.push(self.cond.clone().unwrap());
        base.extend(self.stmts.borrow().clone());

        base.into()
    }
}
