//! # iter-tree

//! This library provide an easy way to transform iterator into tree. This can be useful when building simple parsers to convert a stream of token into a tree of token.

//! It provide two types of tree: 

//! - The default one, [`Tree`](tree::Tree) is based on [`Vec`] from the standard librayry. 

//! - The second one, [`TreeDeque`](tree_deque::TreeDeque) is based on [`VecDeque`](std::collections::VecDeque) from the standard libray. <br/> To get this one, you have to activate the `deque` feature flag.
//!
//! In the future, the goal would be to provide other types of Trees, notably some that separate the token that inited and terminated a branch.
//! 
//! ## Usage
//! 
//! The creattion of a tree is controlled with the [`BranchControl`](controller::BranchControl) enum.
//! 
//! This enum has three variants :
//! 
//! - [`BranchControl::Start`](controller::BranchControl::Start)
//!   - Is used to start nesting the items of the iterator into a new branch.
//! - [`BranchControl::Continue`](controller::BranchControl::Continue)
//!   - Is used to keep the item in the same branch as the previous ones
//! - [`BranchControl::End`](controller::BranchControl::End)
//!   - Is used to get back up to the previous branch to put the next items.
//! 
//! > Note:
//! >
//! > When filling a branch started with [`BranchControl::Start`](controller::BranchControl::Start), no crash or error will happens if the iterator ends before encountering the corresponding [`BranchControl::End`](controller::BranchControl::End).
//! > Similarly, any unmatched [`BranchControl::End`](controller::BranchControl::End) will simply be ignored.
//! >
//! > If you want check for these kind of situations, you can use a trick such as the depth counter showed for example.

//! ### Example
//! 
//! ```rust
//! let mut depth = 0;
//! 
//! let tree = "a+(b+c)+d"
//!     .chars()
//!     .into_iter()
//!     .tree(|&item: &char| match item {
//!         '(' => {
//!             depth += 1;
//!             BranchControl::Start
//!         }
//!         ')' => {
//!             depth -= 1;
//!             BranchControl::End
//!         }
//!         _ => BranchControl::Continue,
//!     })
//!     .collect::<Tree<char>>();
//! 
//! println!("{tree:?}");
//! 
//! assert_eq!(0, depth);
//! ```
//! 
//! ```bash
//! Branch(
//!     [
//!         Leaf(
//!             'a',
//!         ),
//!         Leaf(
//!             '+',
//!         ),
//!         Branch(
//!             [
//!                 Leaf(
//!                     '(',
//!                 ),
//!                 Leaf(
//!                     'b',
//!                 ),
//!                 Leaf(
//!                     '+',
//!                 ),
//!                 Leaf(
//!                     'c',
//!                 ),
//!                 Leaf(
//!                     ')',
//!                 ),
//!             ],
//!         ),
//!         Leaf(
//!             '+',
//!         ),
//!         Leaf(
//!             'd',
//!         ),
//!     ],
//! )
//! ```
//! 
//! ### To go further
//! 
//! Additionally you can create a struct that implements the [`Controller`](controller::Controller) trait to replace the closure from the previous example.
//! 
//! Here is an example of how this can be applied :
//! 
//! ```rust
//! struct StackController<T> {
//!     stack: Vec<T>,
//! }

//! impl<T> StackController<T> {
//!     pub fn is_empty(self) -> bool {
//!         self.stack.is_empty()
//!     }
//! }

//! impl Controller<char> for &mut StackController<char> {
//!     fn control_branch(&mut self, item: &char) -> BranchControl {
//!         let &c = item;
//!         match c {
//!             '<' => {
//!                 self.stack.push(c);
//!                 BranchControl::Start
//!             }
//!             '(' => {
//!                 self.stack.push(c);
//!                 BranchControl::Start
//!             }
//!             '>' => {
//!                 if self.stack.len() > 0 && self.stack.last().unwrap() == &'<' {
//!                     self.stack.pop();
//!                     BranchControl::End
//!                 } else {
//!                     BranchControl::Continue
//!                 }
//!             }
//!             ')' => {
//!                 if self.stack.len() > 0 && self.stack.last().unwrap() == &'(' {
//!                     self.stack.pop();
//!                     BranchControl::End
//!                 } else {
//!                     BranchControl::Continue
//!                 }
//!             }
//!             _ => BranchControl::Continue,
//!         }
//!     }
//! }
//! 
//! 
//! let mut controller = StackController::default();
//! 
//! let _1 = "< ( < > ) >"
//!     .chars()
//!     .tree(&mut controller)
//!     .collect::<Tree<char>>();
//! 
//! assert!(controller.is_empty());
//! 
//! 
//! let mut controller = StackController::default();
//! 
//! let _b = "<(>)".chars().tree(&mut controller).collect::<Tree<_>>();
//! 
//! assert!(!controller.is_empty())
//! ```

mod controller;

#[cfg(feature = "deque")]
pub mod tree_deque;

#[cfg(feature = "vec")]
pub mod tree;

pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[derive(Debug, Default)]
    struct StackController<T> {
        stack: Vec<T>,
    }

    impl<T> StackController<T> {
        pub fn is_empty(self) -> bool {
            self.stack.is_empty()
        }
    }

    impl Controller<char> for &mut StackController<char> {
        fn control_branch(&mut self, item: &char) -> BranchControl {
            let &c = item;
            match c {
                '<' => {
                    self.stack.push(c);
                    BranchControl::Start
                }
                '(' => {
                    self.stack.push(c);
                    BranchControl::Start
                }
                '>' => {
                    if self.stack.len() > 0 && self.stack.last().unwrap() == &'<' {
                        self.stack.pop();
                        BranchControl::End
                    } else {
                        BranchControl::Continue
                    }
                }
                ')' => {
                    if self.stack.len() > 0 && self.stack.last().unwrap() == &'(' {
                        self.stack.pop();
                        BranchControl::End
                    } else {
                        BranchControl::Continue
                    }
                }
                _ => BranchControl::Continue,
            }
        }
    }

    #[cfg(feature = "vec")]
    mod tree {
        use crate::{prelude::*, tests::StackController};

        #[test]
        fn basic_test() {
            let mut depth = 0;

            let tree = "a+(b+c)+d"
                .chars()
                .into_iter()
                .tree(|&item: &char| match item {
                    '(' => {
                        depth += 1;
                        BranchControl::Start
                    }
                    ')' => {
                        depth -= 1;
                        BranchControl::End
                    }
                    _ => BranchControl::Continue,
                })
                .collect::<Tree<char>>();

            println!("{tree:#?}");

            assert_eq!(0, depth)
        }

        #[test]
        fn correct_test() {
            let mut controller = StackController::default();

            let _1 = "< ( < > ) >"
                .chars()
                .tree(&mut controller)
                .collect::<Tree<char>>();

            assert!(controller.is_empty());
        }

        #[test]
        fn incorrect_test() {
            let mut controller = StackController::default();

            let _b = "<(>)".chars().tree(&mut controller).collect::<Tree<_>>();

            assert!(!controller.is_empty())
        }
    }

    #[cfg(feature = "deque")]
    mod tree_deque {
        mod tree {
            use crate::{prelude::*, tests::StackController};

            #[test]
            fn basic_test() {
                let mut depth = 0;
                let tree = "a+b+(c+(d+e)+f)+g"
                    .chars()
                    .into_iter()
                    .tree_deque(|&item: &char| match item {
                        '(' => {
                            depth += 1;
                            BranchControl::Start
                        }
                        ')' => {
                            depth -= 1;
                            BranchControl::End
                        }
                        _ => BranchControl::Continue,
                    })
                    .collect::<TreeDeque<char>>();

                println!("{tree:#?}");

                assert_eq!(0, depth)
            }

            #[test]
            fn correct_test() {
                let mut parser = StackController::default();

                let _1 = "< ( < > ) >"
                    .chars()
                    .tree_deque(&mut parser)
                    .collect::<TreeDeque<char>>();

                //print_type_of(&_1);

                assert!(parser.is_empty());
            }

            #[test]
            fn incorrect_test() {
                let mut parser = StackController::default();

                let _b = "<(>)"
                    .chars()
                    .tree_deque(&mut parser)
                    .collect::<TreeDeque<_>>();

                assert!(!parser.is_empty())
            }
        }
    }
}
