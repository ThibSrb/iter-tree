use std::iter::{once, Once};

use super::TreeDeque;

pub enum IntoIter<T> {
    Unique(Once<T>),
    Multiple(NoneIntoIter<T>),
}

impl<T> From<TreeDeque<T>> for IntoIter<T> {
    fn from(value: TreeDeque<T>) -> Self {
        match value {
            TreeDeque::Leaf(value) => Self::Unique(once(value)),
            TreeDeque::Node(multiple) => Self::Multiple(NoneIntoIter {
                state: NodeIterState::Normal,
                iter: multiple.into_iter(),
            }),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Unique(once) => once.next(),
            Self::Multiple(multiple) => multiple.next(),
        }
    }
}

pub enum NodeIterState<T> {
    Normal,
    Recursion(Box<NoneIntoIter<T>>),
}

pub struct NoneIntoIter<T> {
    state: NodeIterState<T>,
    iter: std::collections::vec_deque::IntoIter<TreeDeque<T>>,
}

impl<T> Iterator for NoneIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            NodeIterState::Normal => {
                let next = self.iter.next()?;

                match next {
                    TreeDeque::Leaf(value) => Some(value),
                    TreeDeque::Node(trees) => {
                        self.state = NodeIterState::Recursion(Box::new(Self {
                            state: NodeIterState::Normal,
                            iter: trees.into_iter(),
                        }));
                        self.next()
                    }
                }
            }
            NodeIterState::Recursion(rec) => {
                let next = rec.next();

                match next {
                    None => {
                        self.state = NodeIterState::Normal;
                        self.next()
                    }
                    value => value,
                }
            }
        }
    }
}

impl<Item> IntoIterator for TreeDeque<Item> {
    type Item = Item;

    type IntoIter = IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
