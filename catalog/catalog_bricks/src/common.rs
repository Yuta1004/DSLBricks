use std::rc::Rc;

use compiler::designer::design::syntax::Rule;

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