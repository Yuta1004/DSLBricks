use std::cell::RefCell;

use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::{Rule, RuleSet};
use compiler::designer::design::DSLGeneratable;
use compiler::bricks::*;

use crate::constraints::ctime::*;

/// # 構造体
///
/// ## 概要
///
/// - Rust の構造体定義を表現します
///
/// ## 性質
/// - DeclaredObject
#[derive(Default)]
#[dslbrick(namespace = std.struct.rust, property = DeclaredObject)]
pub struct Struct {
    #[component(single = Identifiable)]
    id: RefCell<Option<Rule>>,
}

impl DSLBrickDesign for Struct {
    fn design(&self) -> Vec<Rule> {
        let mut rules = vec![
            rule! { Struct -> "struct" id ";" },
            rule! { Struct -> "struct" id r"\{" named_fields r"\}" },
            rule! { Struct -> "struct" id r"\(" fields r"\)" ";" },
            rule! { named_fields -> named_fields "," named_field },
            rule! { named_fields -> named_field },
            rule! { named_fields -> },
            rule! { named_field -> id ":" "TYPE" },
            rule! { fields -> fields "," id },
            rule! { fileds -> id },
            rule! { fields -> },
        ];
        rules.extend(self.id.borrow().clone());
        rules
    }
}

impl DSLBrickAssertion for Struct {
    fn assert(&self) {
        assert!(self.id.borrow().is_some())
    }
}
