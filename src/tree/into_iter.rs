use std::{
    iter::{once, Once},
    marker::PhantomData,
};

use super::{Tree, TreeList};

pub enum IntoIter<Item, Container>
where
    Container: TreeList<Tree<Item, Container>>,
{
    Unique(Once<Item>),
    Multiple(BranchIntoIter<Item, Container>),
}

impl<Item, Container> From<Tree<Item, Container>> for IntoIter<Item, Container>
where
    Container: TreeList<Tree<Item, Container>>,
{
    fn from(value: Tree<Item, Container>) -> Self {
        match value {
            Tree::Leaf(value) => Self::Unique(once(value)),
            Tree::Branch(multiple) => Self::Multiple(BranchIntoIter {
                state: BranchIterState::Normal(PhantomData),
                iter: multiple.into_iter(),
            }),
        }
    }
}

impl<Item, Container> Iterator for IntoIter<Item, Container>
where
    Container: TreeList<Tree<Item, Container>>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Unique(once) => once.next(),
            IntoIter::Multiple(multiple) => multiple.next(),
        }
    }
}

pub enum BranchIterState<Item, Container>
where
    Container: TreeList<Tree<Item, Container>>,
{
    Normal(PhantomData<Item>),
    Recursion(Box<BranchIntoIter<Item, Container>>),
}

pub struct BranchIntoIter<Item, Container>
where
    Container: TreeList<Tree<Item, Container>>,
{
    pub(crate) state: BranchIterState<Item, Container>,
    pub(crate) iter: Container::IntoIter,
}

impl<Item, Container> Iterator for BranchIntoIter<Item, Container>
where
    Container: TreeList<Tree<Item, Container>>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            BranchIterState::Normal(_) => {
                let next = self.iter.next()?;

                match next {
                    Tree::Leaf(value) => Some(value),
                    Tree::Branch(trees) => {
                        self.state = BranchIterState::Recursion(Box::new(Self {
                            state: BranchIterState::Normal(PhantomData),
                            iter: trees.into_iter(),
                        }));
                        self.next()
                    }
                }
            }
            BranchIterState::Recursion(rec) => {
                let next = rec.next();

                match next {
                    None => {
                        self.state = BranchIterState::Normal(PhantomData);
                        self.next()
                    }
                    value => value,
                }
            }
        }
    }
}
