use std::rc::Rc;

use compiler::designer::design::syntax::Rule;

pub trait DSLBrick: DSLBrickMeta + DSLBrickDesign {}

impl<T> DSLBrick for T
where
    T: DSLBrickMeta + DSLBrickDesign,
{}

pub trait DSLBrickMeta {
    fn name(&self) -> &'static str;
    fn start(&self) -> &'static str;
}

pub trait DSLBrickDesign {
    fn design(&self) -> Vec<Rule>;
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
