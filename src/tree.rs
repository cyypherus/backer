pub trait TreeNode: Sized {
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self>;
    fn take_children(&mut self) -> Vec<Self>;

    fn traverse_top_down<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self),
    {
        let mut stack = vec![self];
        while let Some(node) = stack.pop() {
            f(node);
            let children = node.children_mut();
            for child in children.rev() {
                stack.push(child);
            }
        }
    }

    fn traverse_bottom_up<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self),
    {
        let mut stack: Vec<(*mut Self, bool)> = vec![(self, false)];

        // SAFETY: we only dereference pointers when we know they come from &mut references
        // managed in this traversal, and we never alias them simultaneously.
        unsafe {
            while let Some((node_ptr, visited)) = stack.pop() {
                let node = &mut *node_ptr;
                if visited {
                    f(node);
                } else {
                    stack.push((node_ptr, true));
                    let children = node.children_mut();
                    for child in children.rev() {
                        stack.push((child, false));
                    }
                }
            }
        }
    }

    fn map_tree<B, F>(self, mut f: F) -> B
    where
        F: FnMut(Self, Vec<B>) -> B,
    {
        let mut work: Vec<(Self, usize)> = Vec::new();
        let mut stack = vec![self];
        while let Some(mut node) = stack.pop() {
            let children = node.take_children();
            let child_count = children.len();
            work.push((node, child_count));
            for child in children {
                stack.push(child);
            }
        }

        let mut result: Vec<B> = Vec::new();
        for (node, child_count) in work.into_iter().rev() {
            let mapped_children = result.split_off(result.len() - child_count);
            result.push(f(node, mapped_children));
        }
        result.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq, Eq)]
    struct MyNode {
        val: i32,
        children: Vec<MyNode>,
    }

    impl TreeNode for MyNode {
        fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
            self.children.iter_mut()
        }
        fn take_children(&mut self) -> Vec<Self> {
            std::mem::take(&mut self.children)
        }
    }

    fn make_tree() -> MyNode {
        MyNode {
            val: 1,
            children: vec![
                MyNode {
                    val: 2,
                    children: vec![MyNode {
                        val: 4,
                        children: vec![],
                    }],
                },
                MyNode {
                    val: 3,
                    children: vec![MyNode {
                        val: 5,
                        children: vec![],
                    }],
                },
            ],
        }
    }

    #[test]
    fn test_top_down() {
        let mut root = make_tree();
        let mut vals = vec![];
        root.traverse_top_down(|n| vals.push(n.val));
        assert_eq!(vals, vec![1, 2, 4, 3, 5]);
    }

    #[test]
    fn test_bottom_up() {
        let mut root = make_tree();
        let mut vals = vec![];
        root.traverse_bottom_up(|n| vals.push(n.val));
        assert_eq!(vals, vec![4, 2, 5, 3, 1]);
    }

    #[test]
    fn test_mutation() {
        let mut root = make_tree();
        root.traverse_top_down(|n| n.val *= 10);
        let mut vals = vec![];
        root.traverse_top_down(|n| vals.push(n.val));
        assert_eq!(vals, vec![10, 20, 40, 30, 50]);
    }

    #[test]
    fn test_single_node() {
        let mut root = MyNode {
            val: 42,
            children: vec![],
        };
        let mut vals = vec![];
        root.traverse_bottom_up(|n| vals.push(n.val));
        assert_eq!(vals, vec![42]);
    }

    #[test]
    fn test_wide_tree() {
        let mut root = MyNode {
            val: 0,
            children: (1..=10)
                .map(|i| MyNode {
                    val: i,
                    children: vec![],
                })
                .collect(),
        };
        let mut vals = vec![];
        root.traverse_bottom_up(|n| vals.push(n.val));
        assert_eq!(vals, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0]);
    }

    #[test]
    fn test_bottom_up_mutation_clears_children() {
        let mut root = make_tree();
        root.traverse_bottom_up(|n| {
            n.val *= 10;
            n.children.clear();
        });
        assert_eq!(root.val, 10);
        assert!(root.children.is_empty());
    }

    #[test]
    fn test_map_tree_values() {
        let root = make_tree();
        let result: Vec<i32> = root.map_tree(|node, child_results| {
            let mut vals = vec![node.val];
            for child_vals in child_results {
                vals.extend(child_vals);
            }
            vals
        });
        assert_eq!(result, vec![1, 2, 4, 3, 5]);
    }

    #[test]
    fn test_map_tree_type_change() {
        let root = make_tree();
        let result: String = root.map_tree(|node, children: Vec<String>| {
            if children.is_empty() {
                node.val.to_string()
            } else {
                format!("{}({})", node.val, children.join(","))
            }
        });
        assert_eq!(result, "1(2(4),3(5))");
    }

    #[test]
    fn test_map_tree_single_node() {
        let root = MyNode {
            val: 42,
            children: vec![],
        };
        let result: i32 = root.map_tree(|node, children: Vec<i32>| {
            assert!(children.is_empty());
            node.val * 2
        });
        assert_eq!(result, 84);
    }
}
