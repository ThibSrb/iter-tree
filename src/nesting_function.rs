/// The [`Nesting`] enum is used to control the creation of trees.
// This enum has three variants :

/// - [`Nesting::Increase`]
///   - Is used to start nesting the items of the iterator into a new node.
/// - [`Nesting::Maintain`]
///   - Is used to keep the item in the same node as the previous ones
/// - [`Nesting::Decrease`]
///   - Is used to get back up to the previous node to put the next items. If there is no previous branch a new parent branch is then created.

/// If you want to check for these kind of situations, you can use a the depth counter.
pub enum Nesting {
    /// Used to start nesting the items of the iterator into a new node.
    Increase,
    /// Used to keep the item in the same node as the previous ones.
    Maintain,
    /// Used to get back up to the previous node to put the next items. If there is no previous branch a new parent branch is then created.
    Decrease,
}

pub trait NestingFunction<T> {
    fn direction(&mut self, item: &T) -> Nesting;
}

impl<T, F> NestingFunction<T> for F
where
    F: FnMut(&T) -> Nesting,
{
    fn direction(&mut self, item: &T) -> Nesting {
        (self)(item)
    }
}
