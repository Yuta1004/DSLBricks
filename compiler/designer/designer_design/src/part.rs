use crate::syntax::unchecked;

pub trait DSLPart {
    fn design(self) -> unchecked::RuleSet;
}
