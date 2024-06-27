mod into_tree_deque;
mod iter;

use std::collections::VecDeque;

pub use into_tree_deque::IntoTreeDequeExt;

#[derive(Debug, Clone)]
pub enum TreeDeque<Token> {
    Leaf(Token),
    Node(VecDeque<TreeDeque<Token>>),
}
