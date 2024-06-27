use std::collections::VecDeque;

use super::TreeDeque;
use crate::{Nesting, NestingFunction};

pub trait IntoTreeDequeExt<Token> {
    fn into_tree_deque<F>(self, fun: F) -> TreeDeque<Token>
    where
        F: NestingFunction<Token>;
}

impl<Iterable, Token> IntoTreeDequeExt<Token> for Iterable
where
    Iterable: IntoIterator<Item = Token>,
{
    fn into_tree_deque<F>(self, mut fun: F) -> TreeDeque<Token>
    where
        F: NestingFunction<Token>,
    {
        let mut container = VecDeque::new();
        let mut parents = Vec::new();

        for token in self {
            match fun.direction(&token) {
                Nesting::Increase => {
                    let mut parent = VecDeque::new();
                    core::mem::swap(&mut parent, &mut container);
                    parents.push(parent);
                    container.push_back(TreeDeque::Leaf(token));
                }

                Nesting::Maintain => container.push_back(TreeDeque::Leaf(token)),
                Nesting::Decrease => {
                    let mut parent = parents.pop().unwrap_or(VecDeque::new());
                    container.push_back(TreeDeque::Leaf(token));
                    parent.push_back(TreeDeque::Node(container));
                    container = parent;
                }
            }
        }

        while let Some(mut parent) = parents.pop() {
            parent.push_back(TreeDeque::Node(container));
            container = parent;
        }

        TreeDeque::Node(container)
    }
}
