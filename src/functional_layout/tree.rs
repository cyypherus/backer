pub(crate) trait TreeTrait {
    fn children(&self) -> impl DoubleEndedIterator<Item = &Self>;

    fn iter(&self) -> impl Iterator<Item = &Self>
    where
        Self: Sized,
    {
        TreeIter::new(self)
    }

    fn iter_bottom_up(&self) -> impl Iterator<Item = &Self>
    where
        Self: Sized,
    {
        TreeIterBottomUp::new(self)
    }

    fn fold_top_down<B, F>(&self, init: B, f: F) -> B
    where
        Self: Sized,
        F: Fn(B, &Self) -> B,
    {
        self.iter().fold(init, f)
    }

    fn fold_bottom_up<B, F>(&self, init: B, f: F) -> B
    where
        Self: Sized,
        F: Fn(B, &Self) -> B,
    {
        self.iter_bottom_up().fold(init, f)
    }

    fn map_bottom_up<U, F>(&self, f: F) -> U
    where
        Self: Sized,
        F: Fn(&Self, Vec<U>) -> U,
    {
        map_bottom_up(self, f)
    }

    fn fold_with_structure<B, F>(&self, f: F) -> B
    where
        Self: Sized,
        F: Fn(&Self, Vec<B>) -> B,
    {
        fold_with_structure(self, f)
    }
}

pub(crate) trait IntoTreeTrait {
    fn into_data_and_children(self) -> (Self, impl DoubleEndedIterator<Item = Self>)
    where
        Self: Sized;

    fn into_map_bottom_up<U, F>(self, f: F) -> U
    where
        Self: Sized,
        F: Fn(Self, Vec<U>) -> U,
    {
        into_map_bottom_up(self, f)
    }

    fn into_fold_bottom_up<B, F>(self, f: F) -> B
    where
        Self: Sized,
        F: Fn(Self, Vec<B>) -> B,
    {
        into_fold_bottom_up(self, f)
    }
}

enum VisitState<'a, Tree> {
    Visiting(&'a Tree),
    Visited(&'a Tree),
}

enum VisitStateOwned<Tree: IntoTreeTrait> {
    Visiting(Tree),
    Visited((Tree, usize)),
}

pub(crate) struct TreeIter<'a, Tree> {
    stack: Vec<&'a Tree>,
}

impl<'a, Tree> TreeIter<'a, Tree> {
    pub(crate) fn new(root: &'a Tree) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, Tree> Iterator for TreeIter<'a, Tree>
where
    Tree: TreeTrait,
{
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for child in node.children().rev() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}

pub(crate) fn into_map_bottom_up<T, U, F>(root: T, f: F) -> U
where
    T: IntoTreeTrait,
    F: Fn(T, Vec<U>) -> U,
{
    let mut stack: Vec<VisitStateOwned<T>> = vec![VisitStateOwned::Visiting(root)];
    let mut results: Vec<U> = Vec::new();

    while let Some(state) = stack.pop() {
        match state {
            VisitStateOwned::Visiting(node) => {
                let (data, children) = node.into_data_and_children();
                let children: Vec<_> = children.collect();
                let child_count = children.len();

                stack.push(VisitStateOwned::Visited((data, child_count)));

                for child in children.into_iter().rev() {
                    stack.push(VisitStateOwned::Visiting(child));
                }
            }
            VisitStateOwned::Visited((data, child_count)) => {
                let child_results = if child_count == 0 {
                    Vec::new()
                } else {
                    results.split_off(results.len() - child_count)
                };

                let result = f(data, child_results);
                results.push(result);
            }
        }
    }

    results.into_iter().last().unwrap()
}

pub(crate) fn into_fold_bottom_up<T, B, F>(root: T, f: F) -> B
where
    T: IntoTreeTrait,
    F: Fn(T, Vec<B>) -> B,
{
    let mut stack: Vec<VisitStateOwned<T>> = vec![VisitStateOwned::Visiting(root)];
    let mut results: Vec<B> = Vec::new();

    while let Some(state) = stack.pop() {
        match state {
            VisitStateOwned::Visiting(node) => {
                let (data, children) = node.into_data_and_children();
                let children: Vec<_> = children.collect();
                let child_count = children.len();

                stack.push(VisitStateOwned::Visited((data, child_count)));

                for child in children.into_iter().rev() {
                    stack.push(VisitStateOwned::Visiting(child));
                }
            }
            VisitStateOwned::Visited((data, child_count)) => {
                let child_results = if child_count == 0 {
                    Vec::new()
                } else {
                    results.split_off(results.len() - child_count)
                };

                let result = f(data, child_results);
                results.push(result);
            }
        }
    }

    results.into_iter().last().unwrap()
}

pub(crate) fn map_bottom_up<T, U, F>(root: &T, f: F) -> U
where
    T: TreeTrait,
    F: Fn(&T, Vec<U>) -> U,
{
    let mut stack = vec![VisitState::Visiting(root)];
    let mut results: Vec<U> = Vec::new();

    while let Some(state) = stack.pop() {
        match state {
            VisitState::Visiting(node) => {
                let children: Vec<_> = node.children().collect();
                stack.push(VisitState::Visited(node));

                for child in children.iter().rev() {
                    stack.push(VisitState::Visiting(child));
                }
            }
            VisitState::Visited(node) => {
                let children: Vec<_> = node.children().collect();
                let child_count = children.len();

                let child_results = if child_count == 0 {
                    Vec::new()
                } else {
                    results.split_off(results.len() - child_count)
                };

                let result = f(node, child_results);
                results.push(result);
            }
        }
    }

    results.into_iter().last().unwrap()
}

pub(crate) fn fold_with_structure<T, B, F>(root: &T, f: F) -> B
where
    T: TreeTrait,
    F: Fn(&T, Vec<B>) -> B,
{
    let mut stack = vec![VisitState::Visiting(root)];
    let mut results: Vec<B> = Vec::new();

    while let Some(state) = stack.pop() {
        match state {
            VisitState::Visiting(node) => {
                let children: Vec<_> = node.children().collect();
                stack.push(VisitState::Visited(node));

                for child in children.iter().rev() {
                    stack.push(VisitState::Visiting(child));
                }
            }
            VisitState::Visited(node) => {
                let children: Vec<_> = node.children().collect();
                let child_count = children.len();

                let child_results = if child_count == 0 {
                    Vec::new()
                } else {
                    results.split_off(results.len() - child_count)
                };

                let result = f(node, child_results);
                results.push(result);
            }
        }
    }

    results.into_iter().last().unwrap()
}

pub(crate) struct TreeIterBottomUp<'a, Tree> {
    stack: Vec<VisitState<'a, Tree>>,
}

impl<'a, Tree> TreeIterBottomUp<'a, Tree> {
    pub(crate) fn new(root: &'a Tree) -> Self {
        Self {
            stack: vec![VisitState::Visiting(root)],
        }
    }
}

impl<'a, Tree> Iterator for TreeIterBottomUp<'a, Tree>
where
    Tree: TreeTrait,
{
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.stack.pop() {
            match state {
                VisitState::Visiting(node) => {
                    self.stack.push(VisitState::Visited(node));
                    for child in node.children().rev() {
                        self.stack.push(VisitState::Visiting(child));
                    }
                }
                VisitState::Visited(node) => {
                    return Some(node);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestNode {
        value: i32,
        children: Vec<TestNode>,
    }

    impl TestNode {
        fn new(value: i32) -> Self {
            Self {
                value,
                children: Vec::new(),
            }
        }

        fn with_children(value: i32, children: Vec<TestNode>) -> Self {
            Self { value, children }
        }
    }

    impl TreeTrait for TestNode {
        fn children(&self) -> impl DoubleEndedIterator<Item = &Self> {
            self.children.iter()
        }
    }

    impl IntoTreeTrait for TestNode {
        fn into_data_and_children(mut self) -> (Self, impl DoubleEndedIterator<Item = Self>) {
            let children = std::mem::take(&mut self.children);
            (self, children.into_iter())
        }
    }

    #[test]
    fn test_single_node() {
        let root = TestNode::new(1);
        let mut iter = TreeIter::new(&root);

        assert_eq!(iter.next().map(|n| n.value), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_linear_tree() {
        let root =
            TestNode::with_children(1, vec![TestNode::with_children(2, vec![TestNode::new(3)])]);

        let iter = TreeIter::new(&root);
        let values: Vec<i32> = iter.map(|n| n.value).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_branching_tree() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4), TestNode::new(5)]),
                TestNode::new(6),
            ],
        );

        let iter = TreeIter::new(&root);
        let values: Vec<i32> = iter.map(|n| n.value).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_empty_children() {
        let root = TestNode::with_children(1, vec![]);
        let mut iter = TreeIter::new(&root);

        assert_eq!(iter.next().map(|n| n.value), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_ergonomic_iter() {
        let root = TestNode::with_children(1, vec![TestNode::new(2), TestNode::new(3)]);

        let values: Vec<i32> = root.iter().map(|n| n.value).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_ergonomic_bottom_up_iter() {
        let root = TestNode::with_children(1, vec![TestNode::new(2), TestNode::new(3)]);

        let values: Vec<i32> = root.iter_bottom_up().map(|n| n.value).collect();
        assert_eq!(values, vec![2, 3, 1]);
    }

    #[test]
    fn test_bottom_up_single_node() {
        let root = TestNode::new(1);
        let mut iter = TreeIterBottomUp::new(&root);

        assert_eq!(iter.next().map(|n| n.value), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_bottom_up_linear_tree() {
        let root =
            TestNode::with_children(1, vec![TestNode::with_children(2, vec![TestNode::new(3)])]);

        let iter = TreeIterBottomUp::new(&root);
        let values: Vec<i32> = iter.map(|n| n.value).collect();
        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn test_bottom_up_branching_tree() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4), TestNode::new(5)]),
                TestNode::new(6),
            ],
        );

        let iter = TreeIterBottomUp::new(&root);
        let values: Vec<i32> = iter.map(|n| n.value).collect();
        assert_eq!(values, vec![2, 4, 5, 3, 6, 1]);
    }

    #[test]
    fn test_bottom_up_complex_tree() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::with_children(
                    2,
                    vec![
                        TestNode::new(4),
                        TestNode::with_children(5, vec![TestNode::new(8), TestNode::new(9)]),
                    ],
                ),
                TestNode::with_children(3, vec![TestNode::new(6), TestNode::new(7)]),
            ],
        );

        let iter = TreeIterBottomUp::new(&root);
        let values: Vec<i32> = iter.map(|n| n.value).collect();
        assert_eq!(values, vec![4, 8, 9, 5, 2, 6, 7, 3, 1]);
    }

    #[test]
    fn test_compare_traversals() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4)]),
            ],
        );

        let top_down: Vec<i32> = TreeIter::new(&root).map(|n| n.value).collect();
        let bottom_up: Vec<i32> = TreeIterBottomUp::new(&root).map(|n| n.value).collect();

        assert_eq!(top_down, vec![1, 2, 3, 4]);
        assert_eq!(bottom_up, vec![2, 4, 3, 1]);
    }

    #[test]
    fn test_iterator_traits() {
        let root = TestNode::with_children(1, vec![TestNode::new(2), TestNode::new(3)]);

        let count = TreeIter::new(&root).count();
        assert_eq!(count, 3);

        let found = TreeIter::new(&root).find(|n| n.value == 2);
        assert!(found.is_some());
        assert_eq!(found.unwrap().value, 2);

        let bottom_up_count = TreeIterBottomUp::new(&root).count();
        assert_eq!(bottom_up_count, 3);
    }

    #[test]
    fn test_chaining_with_ergonomic_methods() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4), TestNode::new(5)]),
            ],
        );

        let filtered_values: Vec<i32> = root
            .iter()
            .filter(|n| n.value % 2 == 0)
            .map(|n| n.value)
            .collect();
        assert_eq!(filtered_values, vec![2, 4]);

        let bottom_up_sum: i32 = root.iter_bottom_up().map(|n| n.value).sum();
        assert_eq!(bottom_up_sum, 15);
    }

    #[derive(Debug, PartialEq)]
    struct MappedNode {
        doubled_value: i32,
        child_count: usize,
        children: Vec<MappedNode>,
    }

    impl TreeTrait for MappedNode {
        fn children(&self) -> impl DoubleEndedIterator<Item = &Self> {
            self.children.iter()
        }
    }

    impl IntoTreeTrait for MappedNode {
        fn into_data_and_children(mut self) -> (Self, impl DoubleEndedIterator<Item = Self>) {
            let children = std::mem::take(&mut self.children);
            (self, children.into_iter())
        }
    }

    #[test]
    fn test_tree_fold_operations() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4)]),
            ],
        );

        let sum_top_down = root.fold_top_down(0, |acc, node| acc + node.value);
        assert_eq!(sum_top_down, 10);

        let sum_bottom_up = root.fold_bottom_up(0, |acc, node| acc + node.value);
        assert_eq!(sum_bottom_up, 10);

        let product = root.fold_top_down(1, |acc, node| acc * node.value);
        assert_eq!(product, 24);
    }

    #[test]
    fn test_tree_map_transformation() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4)]),
            ],
        );

        let mapped = root.map_bottom_up(|node, children| MappedNode {
            doubled_value: node.value * 2,
            child_count: children.len(),
            children,
        });

        assert_eq!(mapped.doubled_value, 2);
        assert_eq!(mapped.child_count, 2);
        assert_eq!(mapped.children[0].doubled_value, 4);
        assert_eq!(mapped.children[0].child_count, 0);
        assert_eq!(mapped.children[1].doubled_value, 6);
        assert_eq!(mapped.children[1].child_count, 1);
        assert_eq!(mapped.children[1].children[0].doubled_value, 8);
    }

    #[test]
    fn test_fold_with_structure() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4)]),
            ],
        );

        let depth = root.fold_with_structure(|_, child_depths| {
            let max_child_depth = child_depths.into_iter().max().unwrap_or(0);
            max_child_depth + 1
        });

        assert_eq!(depth, 3);

        let node_count =
            root.fold_with_structure(|_, child_counts| 1 + child_counts.into_iter().sum::<i32>());

        assert_eq!(node_count, 4);
    }

    #[test]
    fn test_consuming_operations() {
        let root = TestNode::with_children(
            10,
            vec![
                TestNode::new(20),
                TestNode::with_children(30, vec![TestNode::new(40)]),
            ],
        );

        let mapped: MappedNode =
            root.into_fold_bottom_up(|node, child_results: Vec<MappedNode>| MappedNode {
                doubled_value: node.value * 2,
                child_count: child_results.len(),
                children: child_results,
            });

        assert_eq!(mapped.doubled_value, 20);
        assert_eq!(mapped.child_count, 2);
        assert_eq!(mapped.children[0].doubled_value, 40);
        assert_eq!(mapped.children[1].doubled_value, 60);
        assert_eq!(mapped.children[1].children[0].doubled_value, 80);
    }

    #[test]
    fn test_tree_to_tree_mapping_with_fold_structure() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4), TestNode::new(5)]),
                TestNode::new(6),
            ],
        );

        let mapped: MappedNode =
            root.fold_with_structure(|node, child_results: Vec<MappedNode>| MappedNode {
                doubled_value: node.value * 2,
                child_count: child_results.len(),
                children: child_results,
            });

        assert_eq!(mapped.doubled_value, 2);
        assert_eq!(mapped.child_count, 3);

        assert_eq!(mapped.children[0].doubled_value, 4);
        assert_eq!(mapped.children[0].child_count, 0);

        assert_eq!(mapped.children[1].doubled_value, 6);
        assert_eq!(mapped.children[1].child_count, 2);
        assert_eq!(mapped.children[1].children[0].doubled_value, 8);
        assert_eq!(mapped.children[1].children[1].doubled_value, 10);

        assert_eq!(mapped.children[2].doubled_value, 12);
        assert_eq!(mapped.children[2].child_count, 0);
    }
}
