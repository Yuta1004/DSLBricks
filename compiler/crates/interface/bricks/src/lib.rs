mod attribute;
pub mod prelude;

use std::rc::Rc;

pub use baker::dslbrick;
pub use composer::combine_bricks;

use attribute::*;

pub trait DSLBrick
where
    Self: DSLBrickMeta + DSLBrickDesign + DSLBrickAssertion,
{}

impl<T> DSLBrick for T
where
    T: DSLBrickMeta + DSLBrickDesign + DSLBrickAssertion,
{}

pub trait DSLBrickRc
where
    Self: Default + DSLBrick,
{
    fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn unwrap(self: Rc<Self>) -> Self {
        Rc::into_inner(self).unwrap()
    }
}

impl<T> DSLBrickRc for T
where
    T: Default + DSLBrick,
{}
