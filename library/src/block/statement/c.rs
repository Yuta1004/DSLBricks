use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

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
#[derive(Clone)]
#[impl_constraints(Executable)]
pub struct Block {
    stmts: Vec<Rule>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            stmts: vec![],
        }
    }

    pub fn add_stmt<T>(mut self, stmt: T) -> Block
    where
        T: Executable + 'static,
    {
        self.stmts.push(rule! { stmt -> [stmt] });
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
        assert!(self.stmts.len() > 0);

        let mut base = vec![
            rule! { block -> r"\{" stmts r"\}" },
            rule! { block -> stmt },
            rule! { stmts -> stmts stmt },
            rule! { stmts -> stmt },
        ];
        base.extend(self.stmts.clone());

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
#[derive(Clone)]
#[impl_constraints(Executable)]
pub struct ExprStatement {
    expr: Option<Rule>,
}

impl ExprStatement {
    pub fn new() -> ExprStatement {
        ExprStatement {
            expr: None,
        }
    }

    pub fn set_expr<T>(mut self, expr: T) -> ExprStatement
    where
        T: Calculatable + 'static,
    {
        self.expr = Some(rule! { stmt -> [expr] ";" });
        self
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
#[derive(Clone)]
#[impl_constraints(Executable)]
pub struct If {
    cond: Option<Rule>,
    stmts: Vec<Rule>,
}

impl If {
    pub fn new() -> If {
        If {
            cond: None,
            stmts: vec![],
        }
    }

    pub fn set_cond<T>(mut self, cond: T) -> If
    where
        T: Calculatable + 'static,
    {
        self.cond = Some(rule! { cond -> [cond] });
        self
    }

    pub fn add_stmt<T>(mut self, stmt: T) -> If
    where
        T: Executable + 'static,
    {
        self.stmts.push(rule! { stmt -> [stmt] });
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
        assert!(self.stmts.len() > 0);

        let mut base = vec![
            rule! { if -> "if" r"\(" cond r"\)" stmt },
            rule! { if -> "if" r"\(" cond r"\)" stmt else },
            rule! { else -> "else" if },
            rule! { else -> "else" stmt },
        ];
        base.push(self.cond.clone().unwrap());
        base.extend(self.stmts.clone());

        base.into()
    }
}
