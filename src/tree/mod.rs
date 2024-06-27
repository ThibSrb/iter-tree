mod into_tree;
mod iter;

pub use into_tree::IntoTreeExt;

#[derive(Debug, Clone)]
pub enum Tree<Token> {
    Leaf(Token),
    Node(Vec<Tree<Token>>),
}
