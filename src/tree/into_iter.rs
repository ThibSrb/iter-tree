use std::iter::{once, Once};

use super::Tree;

pub enum IntoIter<T> {
    Unique(Once<T>),
    Multiple(BranchIntoIter<T>),
}

impl<T> From<Tree<T>> for IntoIter<T> {
    fn from(value: Tree<T>) -> Self {
        match value {
            Tree::Leaf(value) => Self::Unique(once(value)),
            Tree::Branch(multiple) => Self::Multiple(BranchIntoIter {
                state: BranchIterState::Normal,
                iter: multiple.into_iter(),
            }),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Unique(once) => once.next(),
            IntoIter::Multiple(multiple) => multiple.next(),
        }
    }
}

pub enum BranchIterState<T> {
    Normal,
    Recursion(Box<BranchIntoIter<T>>),
}

pub struct BranchIntoIter<T> {
    state: BranchIterState<T>,
    iter: std::vec::IntoIter<Tree<T>>,
}

impl<T> Iterator for BranchIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            BranchIterState::Normal => {
                let next = self.iter.next()?;

                match next {
                    Tree::Leaf(value) => Some(value),
                    Tree::Branch(trees) => {
                        self.state = BranchIterState::Recursion(Box::new(Self {
                            state: BranchIterState::Normal,
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
                        self.state = BranchIterState::Normal;
                        self.next()
                    }
                    value => value,
                }
            }
        }
    }
}
