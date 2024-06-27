use super::Tree;
use crate::{Nesting, NestingFunction};

pub trait IntoTreeExt<Token> {
    fn into_tree<F>(self, fun: F) -> Tree<Token>
    where
        F: NestingFunction<Token>;
}

impl<Iterable, Token> IntoTreeExt<Token> for Iterable
where
    Iterable: IntoIterator<Item = Token>,
{
    fn into_tree<F>(self, mut fun: F) -> Tree<Token>
    where
        F: NestingFunction<Token>,
    {
        let mut container = Vec::new();
        let mut parents = Vec::new();

        for token in self {
            match fun.direction(&token) {
                Nesting::Increase => {
                    let mut parent = Vec::new();
                    core::mem::swap(&mut parent, &mut container);
                    parents.push(parent);
                    container.push(Tree::Leaf(token));
                }

                Nesting::Maintain => container.push(Tree::Leaf(token)),
                Nesting::Decrease => {
                    let mut parent = parents.pop().unwrap_or(Vec::new());
                    container.push(Tree::Leaf(token));
                    parent.push(Tree::Node(container));
                    container = parent;
                }
            }
        }

        while let Some(mut parent) = parents.pop() {
            parent.push(Tree::Node(container));
            container = parent;
        }

        Tree::Node(container)
    }
}
