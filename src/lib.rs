//! Generic topological sort algorithm (depth-first)
//!
//! # Examples
//! ```
//!     use szyk::Node;
//!     use szyk;
//!
//!     let result = szyk::sort(
//!         &[
//!             Node::new("wooden pickaxe", vec!["planks", "sticks"], "Pickaxe"),
//!             Node::new("planks", vec!["wood"], "Planks"),
//!             Node::new("sticks", vec!["planks"], "Sticks"),
//!             Node::new("wood", vec![], "Wood"),
//!         ],
//!         "wooden pickaxe",
//!     );
//!     assert_eq!(result, Ok(vec!["Wood", "Planks", "Sticks", "Pickaxe"]));
//! ```

#[derive(Debug, PartialEq)]
pub struct Node<Id, Item>
where
    Id: Copy + Eq,
{
    /// unique identifier
    pub id: Id,
    /// list of dependencies
    pub deps: Vec<Id>,
    /// value stored in the node
    pub value: Item,
}

impl<Id, Item> Node<Id, Item>
where
    Id: Copy + Eq,
{
    pub fn new(id: Id, deps: Vec<Id>, value: Item) -> Self {
        Self { id, deps, value }
    }
}

#[derive(PartialEq, Debug)]
pub enum TopsortError<Id> {
    /// * `Id` - target that wasn't found
    TargetNotFound(Id),
    /// * `Id` - target that depends on itself
    CyclicDependency(Id),
}

fn find_index<Id, Item>(domain: &[Node<Id, Item>], target: Id) -> Result<usize, TopsortError<Id>>
where
    Id: Copy + Eq,
{
    match domain.iter().position(|node| node.id == target) {
        Some(index) => Ok(index),
        None => Err(TopsortError::TargetNotFound(target)),
    }
}

fn visit<Id, Item, F>(
    domain: &[Node<Id, Item>],
    target: Id,
    cb: &mut F,
    visited: &mut Vec<bool>,
    current_path: &mut Vec<Id>,
) -> Result<(), TopsortError<Id>>
where
    Id: Copy + Eq,
    F: FnMut(&Node<Id, Item>),
{
    let index = find_index(domain, target)?;

    if visited[index] {
        return Ok(());
    }

    // detect cyclic dependencies
    if current_path.contains(&target) {
        return Err(TopsortError::CyclicDependency(target));
    }

    // push id to the stack
    current_path.push(target);

    // visit dependencies
    for dep in domain[index].deps.iter() {
        visit(domain, *dep, cb, visited, current_path)?;
    }

    // call callback
    cb(&domain[index]);
    visited[index] = true;

    // pop id from the stack
    current_path.pop();
    Ok(())
}

/// calls `cb` with nodes from `domain` in topological order, ending on the node with id of `target`
///
/// # Examples:
/// ```
///     use szyk::*;
///
///     let mut out = Vec::new();
///     let result = sort_cb(
///         &[
///             Node::new("cat", vec!["dog"], "Garfield"),
///             Node::new("dog", vec![], "Odie"),
///         ],
///         "cat",
///         &mut |node| {
///             out.push(node.id);
///         }
///     );
///     assert_eq!(result, Ok(()));
///     assert_eq!(out, vec!["dog", "cat"]);
/// ```
pub fn sort_cb<Id, Item, F>(
    domain: &[Node<Id, Item>],
    target: Id,
    cb: &mut F,
) -> Result<(), TopsortError<Id>>
where
    Id: Copy + Eq,
    F: FnMut(&Node<Id, Item>),
{
    let size = domain.into_iter().size_hint().0;
    let mut visited: Vec<bool> = Vec::with_capacity(size);
    visited.resize(size, false);
    let mut current_path: Vec<Id> = Vec::new();
    visit(domain, target, cb, &mut visited, &mut current_path)
}

/// returns values of nodes from `domain` in topological order, ending on the node with id of `target`
///
/// # Examples:
/// ```
///     use szyk::*;
///
///     let result = sort(
///         &[
///             Node::new("cat", vec!["dog"], "Garfield"),
///             Node::new("dog", vec![], "Odie"),
///         ],
///         "cat",
///     );
///     assert_eq!(result, Ok(vec!["Odie", "Garfield"]));
/// ```
pub fn sort<Id, Item>(domain: &[Node<Id, Item>], target: Id) -> Result<Vec<Item>, TopsortError<Id>>
where
    Id: Copy + Eq,
    Item: Copy,
{
    let mut out = Vec::new();
    sort_cb(domain, target, &mut |node: &Node<_, _>| {
        out.push(node.value);
    })?;

    Ok(out)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sort_cb_works() {
        let mut out = Vec::new();
        let result = sort_cb(
            &[
                Node::new(1, vec![2, 3], "hello"),
                Node::new(2, vec![], "world"),
                Node::new(3, vec![2], "cat"),
            ],
            1,
            &mut |node: &Node<_, _>| {
                out.push(node.value);
            },
        );
        assert_eq!(result, Ok(()));
        assert_eq!(out, vec!["world", "cat", "hello"]);
    }

    #[test]
    fn sort_works() {
        let result = sort(
            &vec![
                Node::new(1, vec![2, 3], "hello"),
                Node::new(2, vec![], "world"),
                Node::new(3, vec![2], "cat"),
            ],
            1,
        );
        assert_eq!(result, Ok(vec!["world", "cat", "hello"]));
    }

    #[test]
    fn target_not_found() {
        let result = sort(
            &vec![
                Node::new(1, vec![2, 3], "hello"),
                Node::new(2, vec![], "world"),
                Node::new(3, vec![2, 4], "cat"),
            ],
            1,
        );
        assert_eq!(result, Err(TopsortError::TargetNotFound(4)));
    }

    #[test]
    fn cyclic_dependency() {
        let result = sort(
            &vec![
                Node::new(1, vec![2, 3], "hello"),
                Node::new(2, vec![1], "world"),
                Node::new(3, vec![2], "cat"),
            ],
            1,
        );
        assert_eq!(result, Err(TopsortError::CyclicDependency(1)));
    }

    #[test]
    fn empty_domain() {
        let result = sort(&[] as &[Node<i32, i32>], 1);
        assert_eq!(result, Err(TopsortError::TargetNotFound(1)));
    }
}
