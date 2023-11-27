use compiler::designer::constraint::ctime::impl_constraints;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;

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
#[derive(Clone)]
#[impl_constraints(Calculatable)]
pub struct Arithmetic {
    units: Vec<Rule>,
}

impl Arithmetic {
    pub fn new() -> Arithmetic {
        Arithmetic {
            units: vec![],
        }
    }

    pub fn add_unit<T>(mut self, unit: T) -> Arithmetic
    where
        T: Calculatable + 'static,
    {
        self.units.push(rule! { unit -> [unit] });
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
        base.extend(self.units.clone());

        base.into()
    }
}
