pub use r#impl::*;

mod r#impl {
    pub mod __export {
        pub use crate::syntax::unchecked::{Rule, SyntaxElem};
    }

    #[macro_export]
    macro_rules! rule {
        ($left:ident -> $($right:tt)*) => {
            $crate::syntax::macros::__export::Rule::from((
                stringify!($left),
                vec![$(rule!(@call $right),)*]
            ))
        };

        (@call [$design:ident]) => {
            $crate::syntax::macros::__export::SyntaxElem::Hole(unsafe { Rc::from_raw(Rc::into_raw($design)) })
        };

        (@call $nonterm:ident) => {
            $crate::syntax::macros::__export::SyntaxElem::NonTerm(stringify!($nonterm))
        };

        (@call $regex:expr) => {
            $crate::syntax::macros::__export::SyntaxElem::Term($regex)
        };
    }

    pub use rule;
}
