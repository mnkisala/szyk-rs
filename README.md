# szyk-rs
Generic topsort for Rust

[![crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/szyk.svg
[crates-url]: https://crates.io/crates/szyk


## Example
```rust
use szyk::Node;
use szyk;

let result = szyk::sort(
    &[
        Node::new("wooden pickaxe", vec!["planks", "sticks"], "Pickaxe"),
        Node::new("planks", vec!["wood"], "Planks"),
        Node::new("sticks", vec!["planks"], "Sticks"),
        Node::new("wood", vec![], "Wood"),
    ],
    "wooden pickaxe",
);
assert_eq!(result, Ok(vec!["Wood", "Planks", "Sticks", "Pickaxe"]));
```
