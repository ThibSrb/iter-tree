use std::iter::{once, Once};

use super::Tree;

pub enum IntoIter<T> {
    Unique(Once<T>),
    Multiple(NoneIntoIter<T>),
}

impl<T> From<Tree<T>> for IntoIter<T> {
    fn from(value: Tree<T>) -> Self {
        match value {
            Tree::Leaf(value) => Self::Unique(once(value)),
            Tree::Node(multiple) => Self::Multiple(NoneIntoIter {
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
    iter: std::vec::IntoIter<Tree<T>>,
}

impl<T> Iterator for NoneIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            NodeIterState::Normal => {
                let next = self.iter.next()?;

                match next {
                    Tree::Leaf(value) => Some(value),
                    Tree::Node(trees) => {
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

impl<Item> IntoIterator for Tree<Item> {
    type Item = Item;

    type IntoIter = IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
