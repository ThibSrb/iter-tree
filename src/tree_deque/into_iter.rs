use std::{
    collections::VecDeque,
    iter::{once, Once},
};

use super::TreeDeque;

pub enum IntoIter<T> {
    Unique(Once<T>),
    Multiple(BranchIntoIter<T>),
}

impl<T> From<TreeDeque<T>> for IntoIter<T> {
    fn from(value: TreeDeque<T>) -> Self {
        match value {
            TreeDeque::Leaf(value) => Self::Unique(once(value)),
            TreeDeque::Branch(multiple) => Self::Multiple(BranchIntoIter {
                state: BranchIterState::Normal,
                trees: multiple,
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
    trees: VecDeque<TreeDeque<T>>,
}

impl<T> Iterator for BranchIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            BranchIterState::Normal => {
                let next = self.trees.pop_front()?;

                match next {
                    TreeDeque::Leaf(value) => Some(value),
                    TreeDeque::Branch(trees) => {
                        self.state = BranchIterState::Recursion(Box::new(Self {
                            state: BranchIterState::Normal,
                            trees: trees,
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
