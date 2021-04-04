# szyk-rs
Generic topsort for Rust

![crates.io](https://img.shields.io/crates/v/szyk.svg)

## Example
```rust
    use szyk::*;
    let result = topsort_values(
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
