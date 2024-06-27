# iter-tree

This crate provides an easy way to convert between iterators and tree structures. This can be useful when building parsers to convert a stream of token into a tree of token.

It extends iterators with two functions : 

- [`into_tree`](`IntoTreeExt::into_tree`) that transforms an iterator into a [`Tree`].

- [`into_tree_deque`](`IntoTreeDequeExt::into_tree_deque`) that transforms an iterator into a [`TreeDeque`].
  
   To get this one, you have to activate the `deque` feature flag.

Both type of trees implement the [`IntoIterator`] trait.

## Usage

The creation of a tree is controlled with the [`Nesting`] enum.
This enum has three variants :

- [`Nesting::Increase`]
  - Is used to start nesting the items of the iterator into a new node.
- [`Nesting::Maintain`]
  - Is used to keep the item in the same node as the previous ones
- [`Nesting::Decrease`]
  - Is used to get back up to the previous node to put the next items. If there is no previous branch a new parent branch is then created.

If you want to check for these kind of situations, you can use a trick such as the depth counter showed in the below example.

## Example

```rust
use iter_tree::*;

let mut depth = 0;

let before = String::from("a+(b+c)+d");

let tree: Tree<char> = before.chars().into_tree(|&item: &char| match item {
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

let after: String = tree.into_iter().collect();

assert_eq!(before, after);
```

```bash
Node(
    [
        Leaf(
            'a',
        ),
        Leaf(
            '+',
        ),
        Node(
            [
                Leaf(
                    '(',
                ),
                Leaf(
                    'b',
                ),
                Leaf(
                    '+',
                ),
                Leaf(
                    'c',
                ),
                Leaf(
                    ')',
                ),
            ],
        ),
        Leaf(
            '+',
        ),
        Leaf(
            'd',
        ),
    ],
)
```

#### [`NestingFunction`]s

Additionally you can create a struct that implements the [`NestingFunction`] trait to replace the closure from the previous example.

Here is an example of how this can be applied :

```rust
use iter_tree::*;

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
            '>' => {
                if !self.stack.is_empty() && self.stack.last().unwrap() == &'<' {
                    self.stack.pop();
                    Nesting::Decrease
                } else {
                    Nesting::Maintain
                }
            }
            ')' => {
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

let mut parser = StackController::default();

let td = "< ( < > ) >"
    .chars()
    .filter(|c| !c.is_whitespace())
    .into_tree(&mut parser);

assert!(parser.is_empty());
println!("{td:#?}");




let mut parser = StackController::default();

let td = "<(>)".chars().into_tree(&mut parser);

assert!(!parser.is_empty());
println!("{td:#?}");
```

## What's next ?

The goals for the future of this crate includes but are not limited to :

- Adding more methods to build Trees such as for example a `tree_map` and `tree_deque_map` method that would map the item before including it in the Tree.

- Providing other types of Trees, notably some that separate the item that inited and terminated a branch.