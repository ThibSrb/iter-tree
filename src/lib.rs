#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::module_name_repetitions, clippy::ignored_unit_patterns)]
#![forbid(unsafe_code)]
#![doc = include_str!("../DOC.md")]

mod nesting_function;
#[cfg(feature = "vec")]
mod tree;
#[cfg(feature = "deque")]
mod tree_deque;

pub use nesting_function::*;
#[cfg(feature = "vec")]
pub use tree::*;
#[cfg(feature = "deque")]
pub use tree_deque::*;

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::{Nesting, NestingFunction};

    #[derive(Debug, Default)]
    struct StackController<T> {
        stack: Vec<T>,
    }

    impl<T> StackController<T> {
        pub fn is_empty(&self) -> bool {
            self.stack.is_empty()
        }
    }

    impl NestingFunction<char> for &mut StackController<char> {
        fn direction(&mut self, item: &char) -> Nesting {
            let &c = item;
            match c {
                '<' | '(' => {
                    self.stack.push(c);
                    Nesting::Increase
                }
                '>' =>
                {
                    #[allow(clippy::unwrap_used)]
                    if !self.stack.is_empty() && self.stack.last().unwrap() == &'<' {
                        self.stack.pop();
                        Nesting::Decrease
                    } else {
                        Nesting::Maintain
                    }
                }
                ')' =>
                {
                    #[allow(clippy::unwrap_used)]
                    if !self.stack.is_empty() && self.stack.last().unwrap() == &'(' {
                        self.stack.pop();
                        Nesting::Decrease
                    } else {
                        Nesting::Maintain
                    }
                }
                _ => Nesting::Maintain,
            }
        }
    }

    #[cfg(feature = "vec")]
    mod vec {
        use super::*;
        use crate::{IntoTreeExt, Tree};

        #[test]
        fn basic() {
            let tree: Tree<char> = "a * ( (d + b) * c ) + e"
                .chars()
                .filter(|&c: &char| !c.is_whitespace())
                .into_tree(|&c: &char| match c {
                    '(' => Nesting::Increase,
                    ')' => Nesting::Decrease,
                    _ => Nesting::Maintain,
                });

            println!("{tree:#?}");
        }

        #[test]
        fn iter() {
            let before = String::from("a+(b+c)+d");

            let tree = before.chars().into_tree(|&item: &char| match item {
                '(' => Nesting::Increase,
                ')' => Nesting::Decrease,
                _ => Nesting::Maintain,
            });

            let after: String = tree.into_iter().collect();

            assert_eq!(before, after);
        }

        #[test]
        fn with_fn_mut() {
            let mut depth = 0;

            let tree: Tree<char> = "a+(b+c)+d".chars().into_tree(|&item: &char| match item {
                '(' => {
                    depth += 1;
                    Nesting::Increase
                }
                ')' => {
                    depth -= 1;
                    Nesting::Decrease
                }
                _ => Nesting::Maintain,
            });

            assert!(depth == 0);

            println!("{tree:#?}");
        }

        #[test]
        fn correct_stack() {
            let mut parser = StackController::default();

            let td = "< ( < > ) >"
                .chars()
                .filter(|c| !c.is_whitespace())
                .into_tree(&mut parser);

            println!("{td:#?}");

            assert!(parser.is_empty());
        }

        #[test]
        fn incorrect_stack() {
            let mut parser = StackController::default();

            let td = "<(>)".chars().into_tree(&mut parser);

            println!("{td:#?}");

            assert!(!parser.is_empty());
        }
    }

    #[cfg(feature = "deque")]
    mod deque {
        use super::*;
        use crate::{IntoTreeDequeExt, TreeDeque};

        #[test]
        fn basic() {
            let tree: TreeDeque<char> = "a * ( (d + b) * c ) + e"
                .chars()
                .filter(|&c: &char| !c.is_whitespace())
                .into_tree_deque(|&c: &char| match c {
                    '(' => Nesting::Increase,
                    ')' => Nesting::Decrease,
                    _ => Nesting::Maintain,
                });

            println!("{tree:#?}");
        }

        #[test]
        fn iter() {
            let before = String::from("a+(b+c)+d");

            let tree = before.chars().into_tree_deque(|&item: &char| match item {
                '(' => Nesting::Increase,
                ')' => Nesting::Decrease,
                _ => Nesting::Maintain,
            });

            let after: String = tree.into_iter().collect();

            assert_eq!(before, after);
        }

        #[test]
        fn correct_stack() {
            let mut parser = StackController::default();

            let td = "< ( < > ) >"
                .chars()
                .filter(|c| !c.is_whitespace())
                .into_tree_deque(&mut parser);

            println!("{td:#?}");

            assert!(parser.is_empty());
        }

        #[test]
        fn incorrect_stack() {
            let mut parser = StackController::default();

            let td = "<(>)".chars().into_tree_deque(&mut parser);

            println!("{td:#?}");

            assert!(!parser.is_empty());
        }
    }
}
