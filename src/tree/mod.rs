mod adapter;
mod into_iter;
pub use adapter::Treeable;
use into_iter::IntoIter;

#[derive(Debug)]
pub enum Tree<T> {
    Leaf(T),
    Branch(Vec<Tree<T>>),
}

impl<Item> FromIterator<Tree<Item>> for Tree<Item> {
    fn from_iter<T: IntoIterator<Item = Tree<Item>>>(iter: T) -> Self {
        Tree::Branch(Vec::from_iter(iter))
    }
}

impl<Item> IntoIterator for Tree<Item> {
    type Item = Item;

    type IntoIter = IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
