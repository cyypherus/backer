pub trait TreeNode {
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self>;

    fn cata<T, F>(&mut self, mut extract: F) -> Vec<T>
    where
        F: FnMut(&mut Self) -> T,
    {
        let mut result = Vec::new();
        let mut stack = vec![self];
        while let Some(node) = stack.pop() {
            result.push(extract(node));
            let children = node.children_mut();
            for child in children.rev() {
                stack.push(child);
            }
        }
        result
    }

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
                    // First push node marked as visited
                    stack.push((node_ptr, true));
                    // Then push children
                    let children = node.children_mut();
                    for child in children.rev() {
                        stack.push((child, false));
                    }
                }
            }
        }
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
    fn test_catamorphism() {
        let mut root = make_tree();
        let vals = root.cata(|n| n.val);
        assert_eq!(vals, vec![1, 2, 4, 3, 5]);
    }
}
