#[allow(unused)]
#[derive(Debug)]
pub enum BranchControl {
    Start,
    Continue,
    End,
}

pub trait Controller<T> {
    fn control_branch(&mut self, item: &T) -> BranchControl;
}

impl<T, F> Controller<T> for F
where
    F: FnMut(&T) -> BranchControl,
{
    fn control_branch(&mut self, item: &T) -> BranchControl {
        (self)(item)
    }
}
