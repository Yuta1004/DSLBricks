use std::fmt::Debug;

use crate::syntax::unchecked;

pub trait DSLPart
where
    Self: Debug,
{
    fn design(self) -> unchecked::RuleSet;
}
