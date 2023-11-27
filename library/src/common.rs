use std::rc::Rc;

use compiler::designer::design::DSLGeneratable;

pub trait DSLBlock
where
    Self: DSLGeneratable + Sized + 'static
{
    fn new() -> Rc<Self>;

    fn as_dyn(self: Rc<Self>) -> Rc<dyn DSLGeneratable> {
        unsafe {
            Rc::from_raw(Rc::into_raw(self))
        }
    }

    fn unwrap(self: Rc<Self>) -> Self {
        Rc::into_inner(self).unwrap()
    }
}
