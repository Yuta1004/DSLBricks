pub use crate::syntax::{Rule, SyntaxElem};
pub use crate::rule;

#[macro_export]
macro_rules! rule {
    ($left:ident -> $($right:tt)*) => {
        Rule::from((
            stringify!($left),
            vec![$(rule!(@call $right),)*]
        ))
    };

    (@call [$design:ident]) => {
        SyntaxElem::Hole(Box::new($design))
    };

    (@call $nonterm:ident) => {
        SyntaxElem::NonTerm(stringify!($nonterm))
    };

    (@call $regex:expr) => {
        SyntaxElem::Term($regex)
    };
}
