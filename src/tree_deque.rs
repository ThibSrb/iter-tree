use std::collections::VecDeque;

use crate::controller::*;

#[derive(Debug)]
pub enum TreeDeque<T> {
    Leaf(T),
    Branch(VecDeque<TreeDeque<T>>),
}

impl<Item> FromIterator<TreeDeque<Item>> for TreeDeque<Item> {
    fn from_iter<T: IntoIterator<Item = TreeDeque<Item>>>(iter: T) -> Self {
        TreeDeque::Branch(VecDeque::from_iter(iter))
    }
}

pub struct TreeDequeAdapter<I, T, C>
where
    I: Iterator<Item = T>,
    C: Controller<T>,
{
    pub(crate) branching_controller: C,
    pub(crate) iterator: I,
}

impl<I, T, C> TreeDequeAdapter<I, T, C>
where
    I: Iterator<Item = T>,
    C: Controller<T>,
{
    fn branch_control(&mut self, item: &T) -> BranchControl {
        self.branching_controller.control_branch(item)
    }

    fn sub_branch(&mut self, item: T) -> TreeDeque<T> {
        let mut branches = VecDeque::<TreeDeque<T>>::from([TreeDeque::Leaf(item)]);

        loop {
            let next = match self.iterator.next() {
                Some(value) => value,
                None => break,
            };

            match self.branch_control(&next) {
                BranchControl::Start => branches.push_back(self.sub_branch(next)),
                BranchControl::Continue => branches.push_back(TreeDeque::Leaf(next)),
                BranchControl::End => {
                    branches.push_back(TreeDeque::Leaf(next));
                    break;
                }
            }
        }

        TreeDeque::Branch(branches)
    }
}

impl<I, T, C> Iterator for TreeDequeAdapter<I, T, C>
where
    I: Iterator<Item = T>,
    C: Controller<T>,
{
    type Item = TreeDeque<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterator.next()?;

        match self.branch_control(&next) {
            BranchControl::Start => Some(self.sub_branch(next)),
            _ => Some(TreeDeque::Leaf(next)),
        }
    }
}

pub trait TreeDequeable<T>: Iterator<Item = T> + Sized {
    fn tree_deque<C>(self, branching_controller: C) -> TreeDequeAdapter<Self, T, C>
    where
        C: Controller<T>,
    {
        TreeDequeAdapter {
            branching_controller,
            iterator: self,
        }
    }
}

impl<I, T> TreeDequeable<T> for I where I: Iterator<Item = T> + Sized {}
