mod adapter;
mod into_iter;

use std::{fmt::Debug, marker::PhantomData};

pub(crate) use adapter::TreeAdapter;
pub(crate) use into_iter::{IntoIter, BranchIntoIter, BranchIterState};

pub use adapter::Treeable;

pub trait TreeList<T>: IntoIterator<Item = T> {
    fn new() -> Self;
    fn init_with(value: T) -> Self;
    fn push(&mut self, value: T);
    fn len(&self) -> usize;
    fn pop(&mut self) -> Option<T>;
}

pub trait TreeWrapper<Item, List>: Sized
where
    List: TreeList<Tree<Item, List>>,
{
    type Item;

    fn new(value: Tree<Item, List>) -> Self;
    fn take(self) -> Tree<Item, List>;
}

#[derive(Debug)]
pub enum Tree<T, Container>
where
    Container: TreeList<Self>,
{
    Leaf(T),
    Branch(Container),
}

#[derive(Debug)]
pub struct TreeListBackend<Item, List> {
    pub(crate) list: List,
    pub(crate) marker: PhantomData<Item>,
}
