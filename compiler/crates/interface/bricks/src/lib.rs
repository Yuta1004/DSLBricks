pub mod attribute;
pub mod prelude;

use std::rc::Rc;

use designer::design::syntax::RuleSet;
use designer::design::DSLGeneratable;

pub use baker::dslbrick;
pub use composer::combine_bricks;

use attribute::*;

pub trait DSLBrick
where
    Self: Default + DSLBrickMeta + DSLBrickDesign + DSLBrickAssertion,
{
    fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn unwrap(self: Rc<Self>) -> Self {
        Rc::into_inner(self).unwrap()
    }

    fn into(self) -> impl DSLGeneratable {
        struct __DSLBrick<T: DSLBrick>(T);

        impl<T: DSLBrick> DSLGeneratable for __DSLBrick<T> {
            fn name(&self) -> &'static str {
                DSLBrickMeta::name(&self.0)
            }

            fn start(&self) -> &'static str {
                DSLBrickMeta::start(&self.0)
            }

            fn design(&self) -> RuleSet {
                DSLBrickAssertion::assert(&self.0);

                let name = DSLBrickMeta::name(&self.0);
                let design = DSLBrickDesign::design(&self.0);
                (name, design).into()
            }
        }

        __DSLBrick(self)
    }
}

impl<T> DSLBrick for T
where
    T: Default + DSLBrickMeta + DSLBrickDesign + DSLBrickAssertion,
{
    // auto implementation for all DSLBrick
}
