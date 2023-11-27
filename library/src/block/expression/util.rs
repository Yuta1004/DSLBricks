use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use crate::block::common::DSLBlock;
use crate::block::constraints::ctime::*;

/// # 算術式
///
/// ## 概要
///
/// - Calculatable な要素の組み合わせによる算術式を表現します
///
/// ## はめ込み要素
///
/// - unit (Calculatable) : 計算の単位となる構文部品
///
/// ## 性質
///
/// - Calculatable
#[impl_constraints(Calculatable)]
pub struct Arithmetic {
    units: RefCell<Vec<Rule>>,
}

impl DSLBlock for Arithmetic {
    fn new() -> Rc<Self> {
        Rc::new(Arithmetic {
            units: RefCell::new(vec![]),
        })
    }
}

impl Arithmetic {
    pub fn add_unit<T>(self: Rc<Self>, unit: Rc<T>) -> Rc<Self>
    where
        T: DSLBlock + Calculatable,
    {
        self.as_ref().units.borrow_mut().push(rule! { unit -> [{unit.into()}] });
        self
    }
}

impl DSLGeneratable for Arithmetic {
    fn name(&self) -> &'static str {
        "std.expression.util.Arithmetic"
    }

    fn start(&self) -> &'static str {
        "arithmetic"
    }

    fn design(&self) -> RuleSet {
        let mut base = vec![
            rule! { arithmetic -> expr },
            rule! { expr -> expr r"\+" term },
            rule! { expr -> expr r"-" term },
            rule! { expr -> term },
            rule! { term -> term r"\*" fact },
            rule! { term -> term r"/" fact },
            rule! { term -> fact },
            rule! { fact -> r"\(" expr r"\)" },
            rule! { fact -> unit },
        ];
        base.extend(self.units.borrow().clone());

        base.into()
    }
}
