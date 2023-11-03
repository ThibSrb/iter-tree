use std::marker::PhantomData;

use super::{Tree, TreeList};
use crate::controller::{BranchControl, Controller};

pub struct TreeAdapter<Item, Iter, Control, List>
where
    Iter: Iterator<Item = Item>,
    Control: Controller<Item>,
    List: TreeList<Tree<Item, List>>,
{
    pub(crate) branching_controller: Control,
    pub(crate) iterator: Iter,
    marker: PhantomData<List>,
}

impl<Iter, Item, Control, Cont> TreeAdapter<Item, Iter, Control, Cont>
where
    Iter: Iterator<Item = Item>,
    Control: Controller<Item>,
    Cont: TreeList<Tree<Item, Cont>>,
{
    fn branch_control(&mut self, item: &Item) -> BranchControl {
        self.branching_controller.control_branch(item)
    }

    fn sub_branch(&mut self, item: Item) -> Tree<Item, Cont> {
        let mut branches = Cont::init_with(Tree::Leaf(item));

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

impl<Iter, Item, Control, Cont> Iterator for TreeAdapter<Item, Iter, Control, Cont>
where
    Iter: Iterator<Item = Item>,
    Control: Controller<Item>,
    Cont: TreeList<Tree<Item, Cont>>,
{
    type Item = Tree<Item, Cont>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterator.next()?;

        match self.branch_control(&next) {
            BranchControl::Start => Some(self.sub_branch(next)),
            _ => Some(Tree::Leaf(next)),
        }
    }
}

pub trait Treeable<Iter, Container>: Iterator<Item = Iter> + Sized {
    fn tree<C>(self, branching_controller: C) -> TreeAdapter<Iter, Self, C, Container>
    where
        C: Controller<Iter>,
        Container: TreeList<Tree<Iter, Container>>,
    {
        TreeAdapter {
            branching_controller,
            iterator: self,
            marker: PhantomData,
        }
    }
}

impl<I, T, Container> Treeable<T, Container> for I where I: Iterator<Item = T> + Sized {}
