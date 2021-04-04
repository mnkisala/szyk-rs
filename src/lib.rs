pub struct Node<Id, Item>
where
    Id: Copy + PartialEq + Eq,
{
    id: Id,
    deps: Vec<Id>,
    value: Item,
}

impl<Id, Item> Node<Id, Item>
where
    Id: Copy + Clone + PartialEq + Eq,
{
    pub fn new(id: Id, deps: Vec<Id>, value: Item) -> Self {
        Self { id, deps, value }
    }
}

#[derive(PartialEq, Debug)]
pub enum TopsortError<Id> {
    TargetNotFound(Id),
    CyclicDependency(Id),
}

fn visit<Id, Item, F>(
    domain: &[Node<Id, Item>],
    target: Id,
    cb: &mut F,
    visited: &mut Vec<bool>,
    current_path: &mut Vec<Id>,
) -> Result<(), TopsortError<Id>>
where
    Id: Copy + PartialEq + Eq,
    F: FnMut(&Node<Id, Item>),
{
    // find index
    let index = {
        match domain.iter().position(|node| node.id == target) {
            Some(index) => Ok(index),
            None => Err(TopsortError::TargetNotFound(target)),
        }
    }?;

    if visited[index] {
        return Ok(());
    }

    // detect cyclic dependencies
    if current_path.contains(&target) {
        return Err(TopsortError::CyclicDependency(target));
    }
    current_path.push(target);

    // visit dependencies
    for dep in domain[index].deps.iter() {
        visit(domain, *dep, cb, visited, current_path)?;
    }

    // call callback
    cb(&domain[index]);
    visited[index] = true;
    current_path.pop();
    Ok(())
}

pub fn topsort<Id, Item, F>(
    domain: &[Node<Id, Item>],
    target: Id,
    cb: &mut F,
) -> Result<(), TopsortError<Id>>
where
    Id: Copy + Clone + PartialEq + Eq,
    F: FnMut(&Node<Id, Item>),
{
    let size = domain.into_iter().size_hint().0;
    let mut visited: Vec<bool> = Vec::with_capacity(size);
    visited.resize(size, false);
    let mut current_path: Vec<Id> = Vec::new();
    visit(domain, target, cb, &mut visited, &mut current_path)
}

pub fn topsort_values<Id, Item>(
    domain: &[Node<Id, Item>],
    target: Id,
) -> Result<Vec<Item>, TopsortError<Id>>
where
    Id: Copy + PartialEq + Eq,
    Item: Copy,
{
    let mut out = Vec::new();
    topsort(domain, target, &mut |node: &Node<_, _>| {
        out.push(node.value);
    })?;

    Ok(out)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {
        let mut out = Vec::new();
        let result = topsort(
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
    fn target_not_found() {
        let result = topsort_values(
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
        let result = topsort_values(
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
        let result = topsort_values(&[] as &[Node<i32, i32>], 1);
        assert_eq!(result, Err(TopsortError::TargetNotFound(1)));
    }
}
