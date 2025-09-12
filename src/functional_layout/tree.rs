pub(crate) trait IntoTreeTrait {
    fn into_data_and_children(self) -> (Self, impl DoubleEndedIterator<Item = Self>)
    where
        Self: Sized;

    fn into_fold_top_down<U, Context, F>(self, initial_context: Context, f: F) -> U
    where
        Self: Sized,
        F: Fn(Context, Self) -> (U, Vec<Context>),
    {
        let mut stack = vec![(initial_context, self)];
        let mut result = None;

        while let Some((context, node)) = stack.pop() {
            let (data, children) = node.into_data_and_children();
            let children: Vec<_> = children.collect();
            let (node_result, child_contexts) = f(context, data);

            if result.is_none() {
                result = Some(node_result);
            }

            for (child, child_context) in children.into_iter().zip(child_contexts).rev() {
                stack.push((child_context, child));
            }
        }

        result.unwrap()
    }

    fn into_fold_bottom_up<B, F>(self, f: F) -> Self
    where
        Self: Sized,
        F: Fn(Self) -> Self,
    {
        enum VisitState<T> {
            Visiting(T),
            Visited(T),
        }

        let mut stack: Vec<VisitState<Self>> = vec![VisitState::Visiting(self)];
        let mut results: Vec<Self> = Vec::new();

        while let Some(state) = stack.pop() {
            match state {
                VisitState::Visiting(node) => {
                    let (data, children) = node.into_data_and_children();
                    let children: Vec<_> = children.collect();

                    stack.push(VisitState::Visited(data));

                    for child in children.into_iter().rev() {
                        stack.push(VisitState::Visiting(child));
                    }
                }
                VisitState::Visited(data) => {
                    let result = f(data);
                    results.push(result);
                }
            }
        }

        results.into_iter().last().unwrap()
    }

    fn cata<U, F>(self, f: F) -> U
    where
        Self: Sized,
        F: Fn(Self, Vec<U>) -> U,
    {
        enum VisitState<T> {
            Visiting(T),
            Visited((T, usize)),
        }

        let mut stack: Vec<VisitState<Self>> = vec![VisitState::Visiting(self)];
        let mut results: Vec<U> = Vec::new();

        while let Some(state) = stack.pop() {
            match state {
                VisitState::Visiting(node) => {
                    let (data, children) = node.into_data_and_children();
                    let children: Vec<_> = children.collect();
                    let child_count = children.len();

                    stack.push(VisitState::Visited((data, child_count)));

                    for child in children.into_iter().rev() {
                        stack.push(VisitState::Visiting(child));
                    }
                }
                VisitState::Visited((data, child_count)) => {
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

    impl IntoTreeTrait for TestNode {
        fn into_data_and_children(mut self) -> (Self, impl DoubleEndedIterator<Item = Self>) {
            let children = std::mem::take(&mut self.children);
            (self, children.into_iter())
        }
    }

    #[test]
    fn test_into_fold_top_down() {
        #[derive(Debug, PartialEq)]
        struct LayoutNode {
            value: i32,
            area: f32,
        }

        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4)]),
            ],
        );

        let layout = root.into_fold_top_down(100.0, |area, node| {
            let child_area = area / 2.0;
            let child_contexts = vec![child_area; node.children.len()];

            let layout_node = LayoutNode {
                value: node.value,
                area,
            };

            (layout_node, child_contexts)
        });

        assert_eq!(layout.value, 1);
        assert_eq!(layout.area, 100.0);
    }

    #[test]
    fn test_cata() {
        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4)]),
            ],
        );

        let doubled = root
            .cata(|node, children: Vec<i32>| node.value * 2 + children.into_iter().sum::<i32>());
        assert_eq!(doubled, 20);

        let root = TestNode::with_children(10, vec![TestNode::new(20), TestNode::new(30)]);

        let result = root.cata(|node, children: Vec<String>| {
            if children.is_empty() {
                node.value.to_string()
            } else {
                format!("{}({})", node.value, children.join(","))
            }
        });
        assert_eq!(result, "10(20,30)");
    }

    #[test]
    fn test_cata_structure_building() {
        #[derive(Debug, PartialEq)]
        struct MappedNode {
            doubled_value: i32,
            children: Vec<MappedNode>,
        }

        let root = TestNode::with_children(
            1,
            vec![
                TestNode::new(2),
                TestNode::with_children(3, vec![TestNode::new(4), TestNode::new(5)]),
            ],
        );

        let mapped = root.cata(|node, children: Vec<MappedNode>| MappedNode {
            doubled_value: node.value * 2,
            children,
        });

        assert_eq!(mapped.doubled_value, 2);
        assert_eq!(mapped.children.len(), 2);
        assert_eq!(mapped.children[0].doubled_value, 4);
        assert_eq!(mapped.children[0].children.len(), 0);
        assert_eq!(mapped.children[1].doubled_value, 6);
        assert_eq!(mapped.children[1].children.len(), 2);
        assert_eq!(mapped.children[1].children[0].doubled_value, 8);
        assert_eq!(mapped.children[1].children[1].doubled_value, 10);
    }

    #[test]
    fn test_tree_a_to_tree_b_transformation() {
        #[derive(Debug, PartialEq)]
        struct StringNode {
            label: String,
            children: Vec<StringNode>,
        }

        impl StringNode {
            fn new(label: String) -> Self {
                Self {
                    label,
                    children: Vec::new(),
                }
            }

            fn with_children(label: String, children: Vec<StringNode>) -> Self {
                Self { label, children }
            }
        }

        impl IntoTreeTrait for StringNode {
            fn into_data_and_children(mut self) -> (Self, impl DoubleEndedIterator<Item = Self>) {
                let children = std::mem::take(&mut self.children);
                (self, children.into_iter())
            }
        }

        // Create Tree<B> with inline Tree<A> to Tree<B> transformation using cata
        let parent_tree_b = StringNode::with_children(
            "Root".to_string(),
            vec![
                // Use cata to transform nested Tree<A> to Tree<B> inline
                TestNode::with_children(
                    10,
                    vec![
                        TestNode::with_children(20, vec![TestNode::new(25)]),
                        TestNode::new(30),
                    ],
                )
                .cata(|node, children: Vec<StringNode>| StringNode {
                    label: format!("Node_{}", node.value),
                    children,
                }),
                StringNode::new("Other_Child".to_string()),
            ],
        );

        assert_eq!(parent_tree_b.label, "Root");
        assert_eq!(parent_tree_b.children.len(), 2);
        assert_eq!(parent_tree_b.children[0].label, "Node_10");
        assert_eq!(parent_tree_b.children[0].children.len(), 2);
        assert_eq!(parent_tree_b.children[0].children[0].label, "Node_20");
        assert_eq!(parent_tree_b.children[0].children[0].children.len(), 1);
        assert_eq!(
            parent_tree_b.children[0].children[0].children[0].label,
            "Node_25"
        );
        assert_eq!(parent_tree_b.children[0].children[1].label, "Node_30");
        assert_eq!(parent_tree_b.children[1].label, "Other_Child");
    }
}
