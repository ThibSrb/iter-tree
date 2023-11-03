use std::{
    iter::{once, Map},
    marker::PhantomData,
};

use crate::{
    tree::{BranchIntoIter, IntoIter, TreeAdapter, TreeList, TreeListBackend, TreeWrapper},
    Controller, Tree,
};

type Backend<Item> = TreeListBackend<Item, Vec<VecTree<Item>>>;

#[derive(Debug)]
pub struct VecTree<Item>(Tree<Item, Backend<Item>>);

impl<Item, Iter, Control> From<TreeAdapter<Item, Iter, Control, Backend<Item>>> for VecTree<Item>
where
    Iter: Iterator<Item = Item>,
    Control: Controller<Item>,
{
    fn from(value: TreeAdapter<Item, Iter, Control, Backend<Item>>) -> Self {
        let mut res = Backend::new();

        value.for_each(|i| res.push(i));

        match res.len() {
            1 => VecTree(res.pop().unwrap()),
            _ => VecTree(Tree::Branch(res)),
        }
    }
}

impl<Item> TreeWrapper<Item, TreeListBackend<Item, Vec<VecTree<Item>>>> for VecTree<Item> {
    type Item = Item;

    fn new(value: Tree<Item, TreeListBackend<Item, Vec<VecTree<Item>>>>) -> Self {
        Self(value)
    }

    fn take(self) -> Tree<Item, TreeListBackend<Item, Vec<VecTree<Item>>>> {
        self.0
    }
}

impl<Item, Wrapper> IntoIterator for TreeListBackend<Item, Vec<Wrapper>>
where
    Wrapper: TreeWrapper<Item, Self>,
{
    type Item = Tree<Item, Self>;

    type IntoIter = Map<std::vec::IntoIter<Wrapper>, fn(Wrapper) -> Tree<Item, Self>>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter().map(Wrapper::take)
    }
}

impl<Item, Wrapper> TreeList<Tree<Item, Self>> for TreeListBackend<Item, Vec<Wrapper>>
where
    Wrapper: TreeWrapper<Item, Self>,
{
    fn new() -> Self {
        Self {
            list: Vec::new(),
            marker: PhantomData,
        }
    }
    fn init_with(value: Tree<Item, Self>) -> Self {
        Self {
            list: vec![Wrapper::new(value)],
            marker: PhantomData,
        }
    }

    fn push(&mut self, value: Tree<Item, Self>) {
        self.list.push(Wrapper::new(value))
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn pop(&mut self) -> Option<Tree<Item, Self>> {
        self.list.pop().map(|v| v.take())
    }
}

impl<Item> IntoIterator for VecTree<Item> {
    type Item = Item;

    type IntoIter = IntoIter<Item, Backend<Item>>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0 {
            Tree::Leaf(value) => IntoIter::Unique(once(value)),
            Tree::Branch(value) => IntoIter::Multiple(BranchIntoIter {
                state: crate::tree::BranchIterState::Normal(PhantomData),
                iter: value.into_iter(),
            }),
        }
    }
}
