pub mod prelude;

use std::rc::Rc;

use designer::design::syntax::Rule;

pub use baker::dslbrick;
pub use composer::combine_bricks;

pub trait DSLBrick
where
    Self: DSLBrickMeta + DSLBrickDesign + DSLBrickAssertion,
{}

impl<T> DSLBrick for T
where
    T: DSLBrickMeta + DSLBrickDesign + DSLBrickAssertion,
{}

pub trait DSLBrickMeta {
    fn name(&self) -> &'static str;
    fn start(&self) -> &'static str;
    fn components(&self) -> &[&'static str];
}

pub trait DSLBrickDesign {
    fn design(&self) -> Vec<Rule>;
}

pub trait DSLBrickAssertion {
    fn assert(&self);
}

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
