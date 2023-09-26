# iter-tree

This library provide an easy way to transform iterator into tree. This can be useful when building simple parsers to convert a stream of token into a tree of token.

It provide two types of tree: 

- The default one, `Tree` is based on `Vec` from the standard library. 

- The second one is based on `VecDeque` from the standard libray. To get this one you have to activate the `deque` feature flag.

In the future, the goal would be to provide other types of Trees, notably some that separate the token that inited and terminated a branch.

The goals for the future of this crate includes but are not limited to :

- Providing other types of Trees, notably some that separate the item that inited and terminated a branch.
- Adding more methods to build Trees such as for example a `tree_map` and `tree_deque_map` method that would map the item before including it in the Tree.

## Usage

The creation of a tree is controlled with the `BranchControl` enum.
This enum has three variants :

- BranchControl::Start
  - Is used to start nesting the items of the iterator into a new branch.
- BranchControl::Continue
  - Is used to keep the item in the same branch as the previous ones
- BranchControl::End
  - Is used to get back up to the previous branch to put the next items.

> Note:
> 
> 
> When filling a branch started with `BranchControl::Start`, no crash or error will happens if the iterator ends before encountering the corresponding `BranchControl::End`.
> Similarly, any unmatched `BranchControl::End` will simply be ignored.
> 
> If you want to check for these kind of situations, you can use a trick such as the depth counter showed in the below example.

### Example

```rust
use iter_tree::prelude::*;

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

println!("{tree:?}");

assert_eq!(0, depth);
```

```bash
Branch(
    [
        Leaf(
            'a',
        ),
        Leaf(
            '+',
        ),
        Branch(
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

### To go further

Additionally you can create a struct that implements the `Controller` trait to replace the closure from the previous example.
 
Here is an example of how this can be applied :

```rust
use iter_tree::prelude::*;

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


let mut controller = StackController::default();

let _1 = "< ( < > ) >"
    .chars()
    .tree(&mut controller)
    .collect::<Tree<char>>();

assert!(controller.is_empty());

let mut controller = StackController::default();

let _b = "<(>)".chars().tree(&mut controller).collect::<Tree<_>>();

assert!(!controller.is_empty())
```