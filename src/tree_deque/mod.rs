mod adapter;
mod into_iter;

pub use adapter::TreeDequeable;
use into_iter::IntoIter;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum TreeDeque<T> {
    Leaf(T),
    Branch(VecDeque<TreeDeque<T>>),
}

impl<Item> FromIterator<TreeDeque<Item>> for TreeDeque<Item> {
    fn from_iter<T: IntoIterator<Item = TreeDeque<Item>>>(iter: T) -> Self {
        TreeDeque::Branch(VecDeque::from_iter(iter))
    }
}

impl<Item> IntoIterator for TreeDeque<Item> {
    type Item = Item;

    type IntoIter = IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
