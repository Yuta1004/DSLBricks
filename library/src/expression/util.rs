use std::cell::RefCell;
use std::rc::Rc;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

use macros::*;

use crate::constraints::ctime::*;

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
#[derive(Default)]
#[dslblock(namespace = std.expression.util, property = Calculatable)]
pub struct Arithmetic {
    #[component(multiple = Calculatable)]
    unit: RefCell<Vec<Rule>>,
}

impl Arithmetic {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { Arithmetic -> expr },
            rule! { expr -> expr r"\+" term },
            rule! { expr -> expr r"-" term },
            rule! { expr -> term },
            rule! { term -> term r"\*" fact },
            rule! { term -> term r"/" fact },
            rule! { term -> fact },
            rule! { fact -> r"\(" expr r"\)" },
            rule! { fact -> unit },
        ];
        rules.extend(self.unit.borrow().clone());
        rules
    }
}
