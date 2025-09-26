pub trait TreeNode: Sized {
    // fn push_child(&mut self, child: Self);
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self>;
    // fn take_children(&mut self) -> Vec<Self>;
    // fn children(&self) -> impl DoubleEndedIterator<Item = &Self>;

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

    // fn map_into<Target, F>(self, mut f: F) -> Target
    // where
    //     Target: TreeNode,
    //     F: FnMut(Self) -> Target,
    //     Self: Sized,
    // {
    //     fn rec<N, Tgt, Ffn>(mut node: N, f: &mut Ffn) -> Tgt
    //     where
    //         N: TreeNode,
    //         Tgt: TreeNode,
    //         Ffn: FnMut(N) -> Tgt,
    //     {
    //         let children = node.take_children();
    //         let mut tgt = f(node);
    //         for child in children {
    //             let mapped_child = rec(child, f);
    //             tgt.push_child(mapped_child);
    //         }
    //         tgt
    //     }

    //     rec(self, &mut f)
    // }

    // fn map_into<Target, F>(&self, mut f: F) -> Target
    // where
    //     Target: TreeNode,
    //     F: FnMut(&Self) -> Target,
    // {
    //     // We'll do a post-order traversal (iterative) using raw pointers to avoid borrow conflicts.
    //     unsafe {
    //         let root_ptr = self as *const Self;
    //         let mut stack: Vec<(*const Self, bool)> = vec![(root_ptr, false)];
    //         let mut child_counts: Vec<usize> = Vec::new();
    //         let mut mapped: Vec<Target> = Vec::new();

    //         while let Some((node_ptr, visited)) = stack.pop() {
    //             let node = &*node_ptr;
    //             if visited {
    //                 // children already processed: take their mapped nodes and attach to this target
    //                 let n = child_counts.pop().unwrap_or(0);
    //                 let mut children_mapped = mapped.split_off(mapped.len().saturating_sub(n));
    //                 let mut tgt = f(node);
    //                 // attach in-order
    //                 for child in children_mapped.drain(..) {
    //                     tgt.push_child(child);
    //                 }
    //                 mapped.push(tgt);
    //             } else {
    //                 // first time we see this node: gather children pointers, push visited marker
    //                 // and then push children (in reverse so that first child is processed first).
    //                 let mut child_ptrs: Vec<*const Self> = Vec::new();
    //                 for child in node.children() {
    //                     child_ptrs.push(child as *const Self);
    //                 }
    //                 child_counts.push(child_ptrs.len());
    //                 stack.push((node_ptr, true));
    //                 for &cptr in child_ptrs.iter().rev() {
    //                     stack.push((cptr, false));
    //                 }
    //             }
    //         }

    //         mapped
    //             .pop()
    //             .expect("map_into should always produce a result for non-empty tree")
    //     }
    // }
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
        // fn children(&self) -> impl DoubleEndedIterator<Item = &Self> {
        //     self.children.iter()
        // }
        // fn push_child(&mut self, child: Self) {
        //     self.children.push(child)
        // }
        // fn take_children(&mut self) -> Vec<Self> {
        //     std::mem::take(&mut self.children)
        // }
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

    // #[test]
    // fn test_traverse_top_down_no_double_visitation() {
    //     let mut root = make_tree();
    //     let mut visit_count = std::collections::HashMap::new();

    //     root.traverse_top_down(|n| {
    //         let count = visit_count.entry(n.val).or_insert(0);
    //         *count += 1;
    //     });

    //     // Each node should be visited exactly once
    //     for (val, count) in visit_count {
    //         assert_eq!(
    //             count, 1,
    //             "Node with value {} was visited {} times",
    //             val, count
    //         );
    //     }
    // }

    // #[derive(Debug, PartialEq, Eq)]
    // struct OtherNode {
    //     val: String,
    //     children: Vec<OtherNode>,
    // }

    // impl TreeNode for OtherNode {
    //     fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
    //         self.children.iter_mut()
    //     }
    //     // fn children(&self) -> impl DoubleEndedIterator<Item = &Self> {
    //     //     self.children.iter()
    //     // }
    //     // fn push_child(&mut self, child: Self) {
    //     //     self.children.push(child)
    //     // }
    //     // fn take_children(&mut self) -> Vec<Self> {
    //     //     std::mem::take(&mut self.children)
    //     // }
    // }

    // #[test]
    // fn test_map_preserves_structure_nonrecursive() {
    //     let root = make_tree();
    //     // map each integer to an OtherNode with "v:<n>"
    //     let mapped: OtherNode = root.map_into(|n| OtherNode {
    //         val: format!("v:{}", n.val),
    //         children: vec![],
    //     });

    //     // collect mapped values in preorder to verify structure preserved
    //     let mut mapped_clone = mapped;
    //     let values = mapped_clone.cata(|n| n.val.clone());

    //     assert_eq!(
    //         values,
    //         vec![
    //             "v:1".to_string(),
    //             "v:2".to_string(),
    //             "v:4".to_string(),
    //             "v:3".to_string(),
    //             "v:5".to_string()
    //         ]
    //     );
    // }

    // #[test]
    // fn test_map_to_same_type_transforms_values() {
    //     let root = make_tree();
    //     // map MyNode -> MyNode by multiplying values by 10
    //     let mapped: MyNode = root.map_into(|n| MyNode {
    //         val: n.val * 10,
    //         children: vec![],
    //     });

    //     let mut mapped_clone = mapped;
    //     let vals = mapped_clone.cata(|n| n.val);
    //     assert_eq!(vals, vec![10, 20, 40, 30, 50]);
    // }

    // #[test]
    // fn test_map_single_node() {
    //     let single = MyNode {
    //         val: 7,
    //         children: vec![],
    //     };
    //     let mapped: MyNode = single.map_into(|n| MyNode {
    //         val: n.val * 2,
    //         children: vec![],
    //     });
    //     assert_eq!(mapped.val, 14);
    //     assert!(mapped.children.is_empty());
    // }
}
