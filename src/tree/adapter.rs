use super::Tree;
use crate::controller::{BranchControl, Controller};

pub struct TreeAdapter<I, T, C>
where
    I: Iterator<Item = T>,
    C: Controller<T>,
{
    pub(crate) branching_controller: C,
    pub(crate) iterator: I,
}

impl<I, T, C> TreeAdapter<I, T, C>
where
    I: Iterator<Item = T>,
    C: Controller<T>,
{
    fn branch_control(&mut self, item: &T) -> BranchControl {
        self.branching_controller.control_branch(item)
    }

    fn sub_branch(&mut self, item: T) -> Tree<T> {
        let mut branches = Vec::<Tree<T>>::from([Tree::Leaf(item)]);

        loop {
            let next = match self.iterator.next() {
                Some(value) => value,
                None => break,
            };

            match self.branch_control(&next) {
                BranchControl::Start => branches.push(self.sub_branch(next)),
                BranchControl::Continue => branches.push(Tree::Leaf(next)),
                BranchControl::End => {
                    branches.push(Tree::Leaf(next));
                    break;
                }
            }
        }

        Tree::Branch(branches)
    }
}

impl<I, T, C> Iterator for TreeAdapter<I, T, C>
where
    I: Iterator<Item = T>,
    C: Controller<T>,
{
    type Item = Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterator.next()?;

        match self.branch_control(&next) {
            BranchControl::Start => Some(self.sub_branch(next)),
            _ => Some(Tree::Leaf(next)),
        }
    }
}

pub trait Treeable<T>: Iterator<Item = T> + Sized {
    fn tree<C>(self, branching_controller: C) -> TreeAdapter<Self, T, C>
    where
        C: Controller<T>,
    {
        TreeAdapter {
            branching_controller,
            iterator: self,
        }
    }
}

impl<I, T> Treeable<T> for I where I: Iterator<Item = T> + Sized {}
