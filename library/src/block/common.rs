use std::rc::Rc;

use compiler::designer::design::DSLGeneratable;

pub trait DSLBlock
where
    Self: DSLGeneratable + Sized + 'static
{
    fn new() -> Rc<Self>;

    fn into(self: Rc<Self>) -> Rc<dyn DSLGeneratable> {
        unsafe {
            Rc::from_raw(Rc::into_raw(self))
        }
    }
}
