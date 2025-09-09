//! This is a multipass layout crate
//!
//! Zero recursion is used in layout.
//!
//! Layout is performed with two types of passes:
//!
//! Constraint pass: The constraint pass propagates constraints from child to parent
//! Area pass: The area allocation pass propagates available area from parent to child using constraints
//!
//! The two passes are repeated as necessary to allow for dynamic constraints which depend on proposed area
//!
//! Parent constraints *never* override child constraints, parents can only impact child nodes by proposing a different area.

use std::fmt::Debug;
use std::ops::RangeBounds;
use std::rc::Rc;

use crate::mvp::tree::{ConstructionNode, FlatNode, NodeId, Tree};

type AreaReaderFn<T, U> = Box<dyn Fn(Area, &mut T, &mut U) -> Node<T, U>>;
type DynamicNodeFn<T, U> = Box<dyn Fn(&mut T, &mut U) -> Node<T, U>>;
type DrawableFn<T, U> = Box<dyn Fn(Area, &mut T, &mut U)>;
type DimensionFn<T, U> = Option<Rc<dyn Fn(f32, &mut T, &mut U) -> f32>>;
type IntermediateBeforeFn<T, U> = Box<dyn Fn(Area, &mut T, &mut U)>;
type IntermediateAfterFn<T, U> = Box<dyn Fn(&mut T, &mut U)>;

mod tree {
    use std::collections::VecDeque;

    pub(crate) type NodeId = usize;

    pub(crate) struct Tree<F: FlatNode> {
        nodes: Vec<F>,
        pub root_id: NodeId,
    }

    impl<F: FlatNode> Tree<F> {
        pub(crate) fn new<C: ConstructionNode>(root_node: C) -> Self
        where
            F: From<C>,
        {
            let mut tree = Self {
                nodes: Vec::new(),
                root_id: 0,
            };
            tree.add_child_internal(None, root_node);
            tree
        }

        pub(crate) fn top_down(&self, from: NodeId) -> impl Iterator<Item = NodeId> + use<F> {
            let mut stack = vec![from];

            let mut visit_order = Vec::new();

            while let Some(node_id) = stack.pop() {
                visit_order.push(node_id);
                let children = &self.nodes[node_id].children();
                for &child_id in children.iter().rev() {
                    stack.push(child_id);
                }
            }

            visit_order.into_iter()
        }

        pub(crate) fn top_down_depth(
            &self,
            from: NodeId,
        ) -> impl Iterator<Item = (usize, NodeId)> + use<F> {
            let mut stack = vec![(0, from)];

            let mut visit_order = Vec::new();

            while let Some((depth, node_id)) = stack.pop() {
                visit_order.push((depth, node_id));
                let children = &self.nodes[node_id].children();
                for &child_id in children.iter().rev() {
                    stack.push((depth + 1, child_id));
                }
            }
            visit_order.into_iter()
        }

        pub(crate) fn bottom_up(&self, from: NodeId) -> impl Iterator<Item = NodeId> + use<F> {
            let mut stack = vec![(from, false)];
            let mut visit_order = Vec::new();

            while let Some((node_id, visited)) = stack.pop() {
                if visited {
                    visit_order.push(node_id);
                } else {
                    stack.push((node_id, true));
                    let children = &self.nodes[node_id].children();
                    for &child_id in children.iter().rev() {
                        stack.push((child_id, false));
                    }
                }
            }

            visit_order.into_iter()
        }

        pub(crate) fn get_node(&self, node_id: NodeId) -> &F {
            self.nodes.get(node_id).unwrap()
        }

        pub(crate) fn get_node_mut(&mut self, node_id: NodeId) -> &mut F {
            self.nodes.get_mut(node_id).unwrap()
        }
        pub(crate) fn add_child<C: ConstructionNode>(
            &mut self,
            parent_id: NodeId,
            child_node: C,
        ) -> NodeId
        where
            F: From<C>,
        {
            self.add_child_internal(Some(parent_id), child_node)
        }

        #[allow(dead_code)]
        pub(crate) fn replace_node<C: ConstructionNode>(&mut self, node_id: NodeId, new_node: C)
        where
            F: From<C>,
        {
            self.flatten_tree(Some(node_id), None, new_node);
        }

        fn add_child_internal<C: ConstructionNode>(
            &mut self,
            parent_id: Option<NodeId>,
            root_node: C,
        ) -> NodeId
        where
            F: From<C>,
        {
            self.flatten_tree(None, parent_id, root_node)
        }

        fn flatten_tree<C: ConstructionNode>(
            &mut self,
            replace_at: Option<NodeId>,
            parent_id: Option<NodeId>,
            root_node: C,
        ) -> NodeId
        where
            F: From<C>,
        {
            struct WorkItem<C> {
                node: C,
                parent_id: Option<NodeId>,
            }

            let root_id = replace_at.unwrap_or(self.nodes.len());
            let mut queue = VecDeque::new();

            queue.push_back(WorkItem {
                node: root_node,
                parent_id,
            });

            while let Some(mut work_item) = queue.pop_front() {
                let current_node_id = if work_item.parent_id == parent_id {
                    root_id
                } else {
                    self.nodes.len()
                };

                let children = std::mem::take(work_item.node.children_mut());

                if replace_at.is_some() && work_item.parent_id == parent_id {
                    self.nodes[root_id] = F::from(work_item.node);
                } else {
                    self.nodes.push(F::from(work_item.node));
                    if let Some(parent_id) = work_item.parent_id {
                        self.nodes[parent_id].children_mut().push(current_node_id);
                    }
                }

                for child in children {
                    queue.push_back(WorkItem {
                        node: child,
                        parent_id: Some(current_node_id),
                    });
                }
            }

            root_id
        }
    }

    pub(crate) trait FlatNode {
        fn children(&self) -> &Vec<NodeId>;
        fn children_mut(&mut self) -> &mut Vec<NodeId>;
    }

    pub(crate) trait ConstructionNode: Sized {
        fn children_mut(&mut self) -> &mut Vec<Self>;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct TestNode {
            data: i32,
            children: Vec<TestNode>,
        }

        impl ConstructionNode for TestNode {
            fn children_mut(&mut self) -> &mut Vec<Self> {
                &mut self.children
            }
        }

        struct TestFlatNode {
            data: i32,
            children: Vec<NodeId>,
        }

        impl FlatNode for TestFlatNode {
            fn children(&self) -> &Vec<NodeId> {
                &self.children
            }

            fn children_mut(&mut self) -> &mut Vec<NodeId> {
                &mut self.children
            }
        }

        impl From<TestNode> for TestFlatNode {
            fn from(node: TestNode) -> Self {
                TestFlatNode {
                    data: node.data,
                    children: Vec::new(),
                }
            }
        }

        fn test_node(value: i32, children: Vec<TestNode>) -> TestNode {
            TestNode {
                data: value,
                children,
            }
        }

        fn leaf(value: i32) -> TestNode {
            test_node(value, vec![])
        }

        fn create_test_tree() -> Tree<TestFlatNode> {
            let root = test_node(
                1,
                vec![
                    //>
                    test_node(
                        2,
                        vec![
                            //>
                            leaf(4),
                            leaf(5),
                        ],
                    ),
                    leaf(3),
                ],
            );
            Tree::new(root)
        }

        #[test]
        fn test_traversal_styles() {
            let tree = create_test_tree();

            // Test parents first (same as top_down)
            let values: Vec<i32> = tree
                .top_down(tree.root_id)
                .map(|node_id| tree.get_node(node_id).data)
                .collect();
            assert_eq!(values, vec![1, 2, 4, 5, 3]);

            // Test children first (bottom-up)
            let values: Vec<i32> = tree
                .bottom_up(tree.root_id)
                .map(|node_id| tree.get_node(node_id).data)
                .collect();
            assert_eq!(values, vec![4, 5, 2, 3, 1]);
        }

        #[test]
        fn test_filter_chain() {
            let tree = create_test_tree();
            let even_values: Vec<i32> = tree
                .top_down(tree.root_id)
                .filter_map(|node_id| {
                    let node = tree.get_node(node_id);
                    if node.data % 2 == 0 {
                        Some(node.data)
                    } else {
                        None
                    }
                })
                .collect();
            assert_eq!(even_values, vec![2, 4]);
        }

        #[test]
        fn test_fold_chain() {
            let tree = create_test_tree();
            let sum: i32 = tree
                .top_down(tree.root_id)
                .fold(0, |acc, node_id| acc + tree.get_node(node_id).data);
            assert_eq!(sum, 15); // 1+2+4+5+3
        }

        #[test]
        fn test_intuitive_construction() {
            let tree = create_test_tree();

            // Test the structure matches our intuitive construction
            let root_node = tree.get_node(tree.root_id);
            assert_eq!(root_node.data, 1);
            assert_eq!(root_node.children.len(), 2);

            let child1 = root_node.children[0];
            let child2 = root_node.children[1];

            let child1_node = tree.get_node(child1);
            let child2_node = tree.get_node(child2);
            assert_eq!(child1_node.data, 2);
            assert_eq!(child1_node.children.len(), 2);
            assert_eq!(child2_node.data, 3);
            assert_eq!(child2_node.children.len(), 0);
        }

        #[test]
        fn test_replace_node() {
            let mut tree = create_test_tree();

            let root_node = tree.get_node(tree.root_id);
            let child1_id = root_node.children[0];

            let original_child1 = tree.get_node(child1_id);
            assert_eq!(original_child1.data, 2);
            assert_eq!(original_child1.children.len(), 2);

            let replacement_node =
                test_node(42, vec![test_node(100, vec![]), test_node(200, vec![])]);
            tree.replace_node(child1_id, replacement_node);

            let replaced_node = tree.get_node(child1_id);
            assert_eq!(replaced_node.data, 42);
            assert_eq!(replaced_node.children.len(), 2);

            let new_child1 = tree.get_node(replaced_node.children[0]);
            let new_child2 = tree.get_node(replaced_node.children[1]);
            assert_eq!(new_child1.data, 100);
            assert_eq!(new_child2.data, 200);
        }

        #[test]
        fn test_top_down_depth() {
            let tree = create_test_tree();

            let depth_values: Vec<(usize, i32)> = tree
                .top_down_depth(tree.root_id)
                .map(|(depth, node_id)| (depth, tree.get_node(node_id).data))
                .collect();

            // Expected: root(1) at depth 0, then children 2,3 at depth 1, then grandchildren 4,5 at depth 2
            assert_eq!(depth_values, vec![(0, 1), (1, 2), (2, 4), (2, 5), (1, 3)]);

            // Test starting from a different node
            let child_node = tree.get_node(tree.root_id).children[0]; // node with data=2
            let depth_values_from_child: Vec<(usize, i32)> = tree
                .top_down_depth(child_node)
                .map(|(depth, node_id)| (depth, tree.get_node(node_id).data))
                .collect();

            // Starting from node 2, it should be depth 0, with children 4,5 at depth 1
            assert_eq!(depth_values_from_child, vec![(0, 2), (1, 4), (1, 5)]);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Constraint {
    lower: Option<f32>,
    upper: Option<f32>,
}

impl Constraint {
    pub(crate) fn new(lower: Option<f32>, upper: Option<f32>) -> Self {
        assert!(Self::check_constraints(lower, upper));
        Self { lower, upper }
    }

    pub(crate) fn none() -> Self {
        Self::new(None, None)
    }

    pub(crate) fn combine_adjacent_priority(self, other: Self) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        Constraint::new(lower, upper)
    }

    pub(crate) fn combine_sum(self, other: Self, spacing: f32) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(bound)) | (Some(bound), None) => Some(bound + spacing),
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        Constraint::new(lower, upper)
    }

    fn check_constraints(lower: Option<f32>, upper: Option<f32>) -> bool {
        if let (Some(lower_unwrapped), Some(upper_unwrapped)) = (lower, upper) {
            lower_unwrapped <= upper_unwrapped
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SizeConstraints {
    pub(crate) width: Constraint,
    pub(crate) height: Constraint,
    pub(crate) expand_x: bool,
    pub(crate) expand_y: bool,
    pub(crate) x_align: Option<XAlign>,
    pub(crate) y_align: Option<YAlign>,
}

impl SizeConstraints {
    pub(crate) fn should_expand_x(&self) -> bool {
        self.expand_x || self.width.upper.is_none()
    }

    pub(crate) fn should_expand_y(&self) -> bool {
        self.expand_y || self.height.upper.is_none()
    }

    pub(crate) fn combine_parent_child(&self, child: Option<Self>) -> Self {
        SizeConstraints {
            width: Constraint {
                lower: self.width.lower.or(child.and_then(|c| c.width.lower)),
                upper: self.width.upper.or(child.and_then(|c| c.width.upper)),
            },
            height: Constraint {
                lower: self.height.lower.or(child.and_then(|c| c.height.lower)),
                upper: self.height.upper.or(child.and_then(|c| c.height.upper)),
            },
            ..*self
        }
    }
}

impl Default for SizeConstraints {
    fn default() -> Self {
        SizeConstraints {
            width: Constraint::none(),
            height: Constraint::none(),
            expand_x: false,
            expand_y: false,
            x_align: None,
            y_align: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XAlign {
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Top,
    CenterY,
    Bottom,
    Leading,
    CenterX,
    Trailing,
    TopLeading,
    TopCenter,
    TopTrailing,
    CenterTrailing,
    BottomTrailing,
    BottomCenter,
    BottomLeading,
    CenterLeading,
    CenterCenter,
}

impl Align {
    fn axis_aligns(&self) -> (Option<XAlign>, Option<YAlign>) {
        match self {
            Align::TopLeading => (Some(XAlign::Leading), Some(YAlign::Top)),
            Align::TopCenter => (Some(XAlign::Center), Some(YAlign::Top)),
            Align::TopTrailing => (Some(XAlign::Trailing), Some(YAlign::Top)),
            Align::CenterTrailing => (Some(XAlign::Trailing), Some(YAlign::Center)),
            Align::BottomTrailing => (Some(XAlign::Trailing), Some(YAlign::Bottom)),
            Align::BottomCenter => (Some(XAlign::Center), Some(YAlign::Bottom)),
            Align::BottomLeading => (Some(XAlign::Leading), Some(YAlign::Bottom)),
            Align::CenterLeading => (Some(XAlign::Leading), Some(YAlign::Center)),
            Align::CenterCenter => (Some(XAlign::Center), Some(YAlign::Center)),
            Align::Top => (None, Some(YAlign::Top)),
            Align::CenterY => (None, Some(YAlign::Center)),
            Align::Bottom => (None, Some(YAlign::Bottom)),
            Align::Leading => (Some(XAlign::Leading), None),
            Align::CenterX => (Some(XAlign::Center), None),
            Align::Trailing => (Some(XAlign::Trailing), None),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    #[allow(unused)]
    pub(crate) fn zero() -> Self {
        Self::default()
    }
    fn constrained(
        &self,
        constraints: &Option<SizeConstraints>,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
    ) -> Area {
        let Some(constraints) = constraints else {
            return *self;
        };

        let width = match (
            constraints.width.lower,
            if constraints.expand_x {
                None
            } else {
                constraints.width.upper
            },
        ) {
            (None, None) => self.width,
            (None, Some(upper)) => self.width.min(upper),
            (Some(lower), None) => self.width.max(lower),
            (Some(lower), Some(upper)) => self.width.clamp(lower, upper.max(lower)),
        };
        let height = match (
            constraints.height.lower,
            if constraints.expand_y {
                None
            } else {
                constraints.height.upper
            },
        ) {
            (None, None) => self.height,
            (None, Some(upper)) => self.height.min(upper),
            (Some(lower), None) => self.height.max(lower),
            (Some(lower), Some(upper)) => self.height.clamp(lower, upper.max(lower)),
        };

        let x_align = contextual_x_align
            .or(constraints.x_align)
            .unwrap_or(XAlign::Center);
        let y_align = contextual_y_align
            .or(constraints.y_align)
            .unwrap_or(YAlign::Center);

        let x = match x_align {
            XAlign::Leading => self.x,
            XAlign::Trailing => self.x + (self.width - width),
            XAlign::Center => self.x + (self.width * 0.5) - (width * 0.5),
        };

        let y = match y_align {
            YAlign::Top => self.y,
            YAlign::Bottom => self.y + (self.height - height),
            YAlign::Center => self.y + (self.height * 0.5) - (height * 0.5),
        };

        Area {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Padding {
    pub(crate) leading: f32,
    pub(crate) trailing: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

pub(crate) struct NodeConstraints<T, U> {
    pub(crate) width_min: Option<f32>,
    pub(crate) width_max: Option<f32>,
    pub(crate) height_min: Option<f32>,
    pub(crate) height_max: Option<f32>,
    pub(crate) x_align: Option<XAlign>,
    pub(crate) y_align: Option<YAlign>,
    pub(crate) dynamic_height: DimensionFn<T, U>,
    pub(crate) dynamic_width: DimensionFn<T, U>,
    pub(crate) expand_x: bool,
    pub(crate) expand_y: bool,
}

impl<T, U> Debug for NodeConstraints<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeConstraints")
            .field("width_min", &self.width_min)
            .field("width_max", &self.width_max)
            .field("height_min", &self.height_min)
            .field("height_max", &self.height_max)
            .field("x_align", &self.x_align)
            .field("y_align", &self.y_align)
            .field("dynamic_height", &"fn")
            .field("dynamic_width", &"fn")
            .field("expand_x", &self.expand_x)
            .field("expand_y", &self.expand_y)
            .finish()
    }
}

impl<T, U> Clone for NodeConstraints<T, U> {
    fn clone(&self) -> Self {
        Self {
            width_min: self.width_min,
            width_max: self.width_max,
            height_min: self.height_min,
            height_max: self.height_max,
            x_align: self.x_align,
            y_align: self.y_align,
            dynamic_height: self.dynamic_height.clone(),
            dynamic_width: self.dynamic_width.clone(),
            expand_x: self.expand_x,
            expand_y: self.expand_y,
        }
    }
}

impl<T, U> Default for NodeConstraints<T, U> {
    fn default() -> Self {
        Self {
            width_min: None,
            width_max: None,
            height_min: None,
            height_max: None,
            x_align: None,
            y_align: None,
            dynamic_height: None,
            dynamic_width: None,
            expand_x: false,
            expand_y: false,
        }
    }
}

pub(crate) enum NodeType<T, U> {
    Draw(DrawableFn<T, U>),
    Column {
        spacing: f32,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Row {
        spacing: f32,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Stack {
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Padding(Padding),
    Offset {
        x: f32,
        y: f32,
    },
    Space,
    Empty,
    AreaReader {
        func: AreaReaderFn<T, U>,
        expanded: bool,
    },
    Dynamic {
        func: DynamicNodeFn<T, U>,
        expanded: bool,
    },
    Coupled {
        over: bool,
        element: NodeId,
        coupled: NodeId,
    },
    Intermediate {
        before: IntermediateBeforeFn<T, U>,
        after: IntermediateAfterFn<T, U>,
    },
}

impl<T, U> Debug for NodeType<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Draw(_) => write!(f, "Draw"),
            NodeType::Column { .. } => write!(f, "Column"),
            NodeType::Row { .. } => write!(f, "Row"),
            NodeType::Stack { .. } => write!(f, "Stack"),
            NodeType::Padding(_) => write!(f, "Padding"),
            NodeType::Offset { .. } => write!(f, "Offset"),
            NodeType::Space => write!(f, "Space"),
            NodeType::Empty => write!(f, "Empty"),
            NodeType::AreaReader { .. } => write!(f, "AreaReader"),
            NodeType::Dynamic { .. } => write!(f, "Dynamic"),
            NodeType::Coupled { .. } => write!(f, "Coupled"),
            NodeType::Intermediate { .. } => write!(f, "Intermediate"),
        }
    }
}

struct NodeData<T, U> {
    pub(crate) node_type: NodeType<T, U>,
    pub(crate) constraints: NodeConstraints<T, U>,
    pub(crate) children: Vec<NodeId>,
    pub(crate) area: Option<Area>,
    pub(crate) calculated_constraints: Option<SizeConstraints>,
}

impl<T, U> FlatNode for NodeData<T, U> {
    fn children(&self) -> &Vec<NodeId> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.children
    }
}

pub struct Node<T, U> {
    pub(crate) node_type: NodeType<T, U>,
    pub(crate) constraints: NodeConstraints<T, U>,
    pub children: Vec<Node<T, U>>,
}

impl<T, U> ConstructionNode for Node<T, U> {
    fn children_mut(&mut self) -> &mut Vec<Self> {
        &mut self.children
    }
}

impl<T, U> From<Node<T, U>> for NodeData<T, U> {
    fn from(data: Node<T, U>) -> Self {
        Self {
            node_type: data.node_type,
            constraints: data.constraints,
            children: Vec::new(),
            area: None,
            calculated_constraints: None,
        }
    }
}

impl<T, U> Node<T, U> {
    pub(crate) fn new(node_type: NodeType<T, U>) -> Self {
        Self {
            node_type,
            constraints: NodeConstraints::default(),
            children: Vec::new(),
        }
    }

    fn extract_bounds<R: RangeBounds<f32>>(range: R) -> (Option<f32>, Option<f32>) {
        let min = match range.start_bound() {
            std::ops::Bound::Included(bound) | std::ops::Bound::Excluded(bound) => Some(*bound),
            std::ops::Bound::Unbounded => None,
        };
        let max = match range.end_bound() {
            std::ops::Bound::Included(bound) | std::ops::Bound::Excluded(bound) => Some(*bound),
            std::ops::Bound::Unbounded => None,
        };
        (min, max)
    }

    pub(crate) fn with_children(mut self, children: Vec<Node<T, U>>) -> Self {
        self.children = children
            .into_iter()
            .filter(|n| !matches!(n.node_type, NodeType::Empty))
            .collect();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.constraints.width_min = Some(width);
        self.constraints.width_max = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.constraints.height_min = Some(height);
        self.constraints.height_max = Some(height);
        self
    }

    pub fn width_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (width_min, width_max) = Self::extract_bounds(range);
        self.constraints.width_min = width_min;
        self.constraints.width_max = width_max;
        self
    }

    pub fn height_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (height_min, height_max) = Self::extract_bounds(range);
        self.constraints.height_min = height_min;
        self.constraints.height_max = height_max;
        self
    }

    pub fn expand(mut self) -> Self {
        self.constraints.expand_x = true;
        self.constraints.expand_y = true;
        self
    }

    pub fn expand_x(mut self) -> Self {
        self.constraints.expand_x = true;
        self
    }

    pub fn expand_y(mut self) -> Self {
        self.constraints.expand_y = true;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        let (x_align, y_align) = align.axis_aligns();
        if let Some(x) = x_align {
            self.constraints.x_align = Some(x);
        }
        if let Some(y) = y_align {
            self.constraints.y_align = Some(y);
        }
        self
    }

    pub fn pad(self, amount: f32) -> Node<T, U> {
        Node::new(NodeType::Padding(Padding {
            leading: amount,
            trailing: amount,
            top: amount,
            bottom: amount,
        }))
        .with_children(vec![self])
    }

    pub fn pad_x(self, amount: f32) -> Self {
        Node::new(NodeType::Padding(Padding {
            leading: amount,
            trailing: amount,
            top: 0.,
            bottom: 0.,
        }))
        .with_children(vec![self])
    }

    pub fn pad_y(self, amount: f32) -> Self {
        Node::new(NodeType::Padding(Padding {
            leading: 0.,
            trailing: 0.,
            top: amount,
            bottom: amount,
        }))
        .with_children(vec![self])
    }

    pub fn pad_top(self, amount: f32) -> Self {
        Node::new(NodeType::Padding(Padding {
            leading: 0.,
            trailing: 0.,
            top: amount,
            bottom: 0.,
        }))
        .with_children(vec![self])
    }

    pub fn pad_bottom(self, amount: f32) -> Self {
        Node::new(NodeType::Padding(Padding {
            leading: 0.,
            trailing: 0.,
            top: 0.,
            bottom: amount,
        }))
        .with_children(vec![self])
    }

    pub fn pad_leading(self, amount: f32) -> Self {
        Node::new(NodeType::Padding(Padding {
            leading: amount,
            trailing: 0.,
            top: 0.,
            bottom: 0.,
        }))
        .with_children(vec![self])
    }

    pub fn pad_trailing(self, amount: f32) -> Self {
        Node::new(NodeType::Padding(Padding {
            leading: 0.,
            trailing: amount,
            top: 0.,
            bottom: 0.,
        }))
        .with_children(vec![self])
    }

    pub fn offset(self, x: f32, y: f32) -> Node<T, U> {
        Node::new(NodeType::Offset { x, y }).with_children(vec![self])
    }

    pub fn offset_x(self, x: f32) -> Node<T, U> {
        Node::new(NodeType::Offset { x, y: 0. }).with_children(vec![self])
    }

    pub fn offset_y(self, y: f32) -> Node<T, U> {
        Node::new(NodeType::Offset { x: 0., y }).with_children(vec![self])
    }

    pub fn attach_under(self, node: Node<T, U>) -> Node<T, U> {
        Node::new(NodeType::Coupled {
            over: false,
            element: 1,
            coupled: 0,
        })
        .with_children(vec![node, self])
    }

    pub fn attach_over(self, node: Node<T, U>) -> Node<T, U> {
        Node::new(NodeType::Coupled {
            over: true,
            element: 0,
            coupled: 1,
        })
        .with_children(vec![self, node])
    }

    pub fn dynamic_width(mut self, f: impl Fn(f32, &mut T, &mut U) -> f32 + 'static) -> Self {
        self.constraints.dynamic_width = Some(Rc::new(f));
        self
    }

    pub fn dynamic_height(mut self, f: impl Fn(f32, &mut T, &mut U) -> f32 + 'static) -> Self {
        self.constraints.dynamic_height = Some(Rc::new(f));
        self
    }

    pub fn aspect_width(mut self, ratio: f32) -> Self {
        self.constraints.dynamic_width = Some(Rc::new(move |height, _, _| height * ratio));
        self
    }

    pub fn aspect_height(mut self, ratio: f32) -> Self {
        self.constraints.dynamic_height = Some(Rc::new(move |width, _, _| width / ratio));
        self
    }

    pub fn min_height(self, available_area: Area, t: &mut T, u: &mut U) -> Option<f32> {
        let mut layout = Layout::new(self);
        layout.layout_and_expand(layout.tree.root_id, available_area, t, u);
        layout
            .tree
            .get_node(layout.tree.root_id)
            .calculated_constraints
            .and_then(|constraints| constraints.height.lower)
    }

    pub fn min_width(self, available_area: Area, t: &mut T, u: &mut U) -> Option<f32> {
        let mut layout = Layout::new(self);
        layout.layout_and_expand(layout.tree.root_id, available_area, t, u);
        layout
            .tree
            .get_node(layout.tree.root_id)
            .calculated_constraints
            .and_then(|constraints| constraints.width.lower)
    }
}

pub struct Layout<T, U> {
    tree: Tree<NodeData<T, U>>,
}

impl<T, U> std::fmt::Debug for Layout<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_node_structure<T, U>(
            tree: &Tree<NodeData<T, U>>,
            f: &mut std::fmt::Formatter<'_>,
            node_id: NodeId,
            indent_level: usize,
        ) -> std::fmt::Result {
            let indent = "   |   ".repeat(indent_level);
            let node = tree.get_node(node_id);

            if let Some(area) = node.area {
                writeln!(
                    f,
                    "{}x:{}, y:{}, w:{}, h:{}",
                    indent, area.x, area.y, area.width, area.height
                )?;
            }
            if let Some(constraints) = node.calculated_constraints {
                writeln!(
                    f,
                    "{}cwl:{:?}, cwu:{:?}, chl:{:?}, chu:{:?}",
                    indent,
                    constraints.width.lower,
                    constraints.width.upper,
                    constraints.height.lower,
                    constraints.height.upper
                )?;
            }
            writeln!(
                f,
                "{}wl:{:?}, wu:{:?}, hl:{:?}, hu:{:?}",
                indent,
                node.constraints.width_min,
                node.constraints.width_max,
                node.constraints.height_min,
                node.constraints.height_max
            )?;
            match &node.node_type {
                NodeType::Draw(_) => writeln!(f, "{}Draw", indent)?,
                NodeType::Column {
                    spacing,
                    x_align,
                    y_align,
                } => writeln!(
                    f,
                    "{}Column(spacing: {:?}, x_align: {:?}, y_align: {:?})",
                    indent, spacing, x_align, y_align
                )?,
                NodeType::Row {
                    spacing,
                    x_align,
                    y_align,
                } => writeln!(
                    f,
                    "{}Row(spacing: {:?}, x_align: {:?}, y_align: {:?})",
                    indent, spacing, x_align, y_align
                )?,
                NodeType::Stack { x_align, y_align } => writeln!(
                    f,
                    "{}Stack(x_align: {:?}, y_align: {:?})",
                    indent, x_align, y_align
                )?,
                NodeType::Padding(_) => writeln!(f, "{}Padding", indent)?,
                NodeType::Offset { x, y } => writeln!(f, "{}Offset(x: {}, y: {})", indent, x, y)?,
                NodeType::Space => writeln!(f, "{}Space", indent)?,
                NodeType::Empty => writeln!(f, "{}Empty", indent)?,
                NodeType::AreaReader { .. } => writeln!(f, "{}AreaReader", indent)?,
                NodeType::Dynamic { .. } => {
                    writeln!(f, "{}Dynamic (expanded)", indent)?;
                }
                NodeType::Coupled {
                    over,
                    element,
                    coupled,
                } => writeln!(
                    f,
                    "{}Coupled(over: {}, element: {}, coupled: {})",
                    indent, over, element, coupled
                )?,
                NodeType::Intermediate { .. } => writeln!(f, "{}Intermediate", indent,)?,
            }

            for &child_id in node.children() {
                fmt_node_structure(tree, f, child_id, indent_level + 1)?;
            }

            Ok(())
        }
        fmt_node_structure(&self.tree, f, self.tree.root_id, 0)
    }
}

impl<T, U> Layout<T, U> {
    pub fn new(root_node: Node<T, U>) -> Self {
        Self {
            tree: Tree::new(root_node),
        }
    }

    #[cfg(test)]
    pub fn debug_visualize(&mut self, bounds: Area, state: &mut T, ui_state: &mut U) {
        use std::cell::RefCell;
        use std::rc::Rc;

        self.layout_and_expand(self.tree.root_id, bounds, state, ui_state);
        println!("{:?}", &self);

        fn find_all_draw_nodes<T, U>(
            tree: &Tree<NodeData<T, U>>,
            root_id: NodeId,
            draw_nodes: &mut Vec<NodeId>,
        ) {
            let mut stack = vec![root_id];

            while let Some(node_id) = stack.pop() {
                let node = tree.get_node(node_id);
                if matches!(node.node_type, NodeType::Draw(_)) {
                    draw_nodes.push(node_id);
                }
                for &child_id in tree.get_node(node_id).children() {
                    stack.push(child_id);
                }
            }
        }

        fn replace_draw_nodes_with_debug<T, U>(
            tree: &mut Tree<NodeData<T, U>>,
            draw_node_indices: Vec<NodeId>,
            grid: Rc<RefCell<Vec<Vec<char>>>>,
            bounds: Area,
            scale_x: f32,
            scale_y: f32,
            chars: [char; 7],
            char_counter: Rc<RefCell<usize>>,
        ) {
            for node_id in draw_node_indices {
                let grid_clone = grid.clone();
                let char_counter_clone = char_counter.clone();

                tree.get_node_mut(node_id).node_type =
                    NodeType::Draw(Box::new(move |area, _state, _ui_state| {
                        if area.width > 0.0 && area.height > 0.0 {
                            let start_x = ((area.x - bounds.x) / scale_x) as usize;
                            let start_y = ((area.y - bounds.y) / scale_y) as usize;
                            let end_x = (((area.x + area.width) - bounds.x) / scale_x) as usize;
                            let end_y = (((area.y + area.height) - bounds.y) / scale_y) as usize;

                            let mut counter = char_counter_clone.borrow_mut();
                            let ch = chars[*counter % chars.len()];
                            *counter += 1;

                            let mut grid_ref = grid_clone.borrow_mut();
                            for y in start_y..end_y.min(grid_ref.len()) {
                                for x in start_x..end_x.min(grid_ref[0].len()) {
                                    grid_ref[y][x] = ch;
                                }
                            }
                        }
                    }));
            }
        }

        let scale_x = 2.5;
        let scale_y = 7.0;
        let width = (bounds.width / scale_x) as usize;
        let height = (bounds.height / scale_y) as usize;

        let grid = Rc::new(RefCell::new(vec![vec![' '; width]; height]));
        let chars = ['█', '▒', '░', '▪', '●', '■', '='];
        let char_counter = Rc::new(RefCell::new(0));

        // Find all Draw nodes recursively, including those in Coupled structures
        let mut draw_node_indices = Vec::new();
        find_all_draw_nodes(&self.tree, self.tree.root_id, &mut draw_node_indices);

        // Replace all Draw nodes with debug draw functions
        replace_draw_nodes_with_debug(
            &mut self.tree,
            draw_node_indices,
            grid.clone(),
            bounds,
            scale_x,
            scale_y,
            chars,
            char_counter.clone(),
        );

        // Now draw normally - this will call our debug functions
        self.draw(bounds, state, ui_state);

        // Print the result
        let final_grid = grid.borrow();
        println!("Layout visualization ({}x{}):", bounds.width, bounds.height);
        println!("┌{}┐", "─".repeat(width));
        for row in final_grid.iter() {
            print!("│");
            for cell in row {
                print!("{}", cell);
            }
            println!("│");
        }
        println!("└{}┘", "─".repeat(width));
    }

    pub fn draw(&mut self, available_area: Area, state: &mut T, ui_state: &mut U) {
        self.layout_and_expand(self.tree.root_id, available_area, state, ui_state);
        self.draw_iterative(state, ui_state);
    }

    fn layout_iterative(
        &mut self,
        from: NodeId,
        available_area: Area,
        state: &mut T,
        ui_state: &mut U,
    ) {
        for node_id in self.tree.bottom_up(from) {
            let constraints = self.calculate_node_constraints(node_id, state, ui_state);
            self.tree.get_node_mut(node_id).calculated_constraints = Some(constraints);
        }

        self.tree.get_node_mut(from).area = Some(available_area.constrained(
            &self.tree.get_node(from).calculated_constraints,
            None,
            None,
        ));

        for node_id in self.tree.top_down(from) {
            self.allocate_node_area(node_id, state, ui_state);
        }
    }

    fn layout_and_expand(
        &mut self,
        from: NodeId,
        available_area: Area,
        state: &mut T,
        ui_state: &mut U,
    ) {
        for _ in 0..2 {
            self.expand_dynamic_nodes(from, state, ui_state);
            self.layout_iterative(from, available_area, state, ui_state);
            self.expand_area_reader_nodes(from, state, ui_state);
        }
    }

    fn expand_dynamic_nodes(&mut self, from: NodeId, state: &mut T, ui_state: &mut U) {
        let mut stack = vec![from];

        while let Some(node_id) = stack.pop() {
            let expansion_result = if let NodeType::Dynamic {
                func,
                expanded: expanded @ false,
            } = &mut self.tree.get_node_mut(node_id).node_type
            {
                *expanded = true;
                let construction_node = func(state, ui_state);
                let new_child = self.tree.add_child(node_id, construction_node);
                Some(new_child)
            } else {
                None
            };

            if let Some(computed_id) = expansion_result {
                stack.push(computed_id);
            } else {
                let children = self.tree.get_node(node_id).children().clone();
                for &child_id in children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }
    }

    fn expand_area_reader_nodes(&mut self, from: NodeId, state: &mut T, ui_state: &mut U) {
        let mut stack = vec![from];

        while let Some(node_id) = stack.pop() {
            let area = self.tree.get_node(node_id).area;
            let expansion_result = if let NodeType::AreaReader {
                func,
                expanded: expanded @ false,
            } = &mut self.tree.get_node_mut(node_id).node_type
            {
                if let Some(area) = area {
                    *expanded = true;
                    let construction_node = func(area, state, ui_state);
                    let new_child = self.tree.add_child(node_id, construction_node);
                    self.layout_and_expand(new_child, area, state, ui_state);
                    Some(new_child)
                } else {
                    eprintln!("Unexpected area reader expansion without area {:?}", self);
                    None
                }
            } else {
                None
            };

            if let Some(computed_id) = expansion_result {
                stack.push(computed_id);
            } else {
                let children = self.tree.get_node(node_id).children().clone();
                for &child_id in children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }
    }

    fn calculate_node_constraints(
        &self,
        node_id: NodeId,
        state: &mut T,
        ui_state: &mut U,
    ) -> SizeConstraints {
        let node = &self.tree.get_node(node_id);

        let self_constraints = SizeConstraints {
            width: self
                .tree
                .get_node(node_id)
                .area
                .and_then(|area| {
                    node.constraints.dynamic_width.as_ref().map(|f| {
                        let w = f(area.height, state, ui_state);
                        Constraint::new(Some(w), Some(w))
                    })
                })
                .unwrap_or(Constraint::new(
                    node.constraints.width_min,
                    node.constraints.width_max,
                )),
            height: self
                .tree
                .get_node(node_id)
                .area
                .and_then(|area| {
                    node.constraints.dynamic_height.as_ref().map(|f| {
                        let h = f(area.width, state, ui_state);
                        Constraint::new(Some(h), Some(h))
                    })
                })
                .unwrap_or(Constraint::new(
                    node.constraints.height_min,
                    node.constraints.height_max,
                )),
            expand_x: node.constraints.expand_x,
            expand_y: node.constraints.expand_y,
            x_align: node.constraints.x_align,
            y_align: node.constraints.y_align,
        };

        match &node.node_type {
            NodeType::Column { spacing, .. } => self_constraints.combine_parent_child(
                self.tree
                    .get_node(node_id)
                    .children
                    .iter()
                    .filter_map(|child| self.tree.get_node(*child).calculated_constraints)
                    .fold(
                        Option::<SizeConstraints>::None,
                        |current, child_constraints| {
                            if let Some(current) = current {
                                Some(SizeConstraints {
                                    width: current
                                        .width
                                        .combine_adjacent_priority(child_constraints.width),
                                    height: current
                                        .height
                                        .combine_sum(child_constraints.height, *spacing),
                                    ..Default::default()
                                })
                            } else {
                                Some(child_constraints)
                            }
                        },
                    ),
            ),
            NodeType::Row { spacing, .. } => self_constraints.combine_parent_child(
                self.tree
                    .get_node(node_id)
                    .children
                    .iter()
                    .filter_map(|child| self.tree.get_node(*child).calculated_constraints)
                    .fold(
                        Option::<SizeConstraints>::None,
                        |current, child_constraints| {
                            if let Some(current) = current {
                                Some(SizeConstraints {
                                    width: current
                                        .width
                                        .combine_sum(child_constraints.width, *spacing),
                                    height: current
                                        .height
                                        .combine_adjacent_priority(child_constraints.height),
                                    ..Default::default()
                                })
                            } else {
                                Some(child_constraints)
                            }
                        },
                    ),
            ),
            NodeType::Stack { .. } => self_constraints.combine_parent_child(
                self.tree
                    .get_node(node_id)
                    .children
                    .iter()
                    .filter_map(|child| self.tree.get_node(*child).calculated_constraints)
                    .fold(
                        Option::<SizeConstraints>::None,
                        |current, child_constraints| {
                            if let Some(current) = current {
                                Some(SizeConstraints {
                                    width: current
                                        .width
                                        .combine_adjacent_priority(child_constraints.width),
                                    height: current
                                        .height
                                        .combine_adjacent_priority(child_constraints.height),
                                    ..Default::default()
                                })
                            } else {
                                Some(child_constraints)
                            }
                        },
                    ),
            ),
            NodeType::Padding(padding) => {
                self_constraints.combine_parent_child(node.children.first().and_then(|child_id| {
                    self.tree
                        .get_node(*child_id)
                        .calculated_constraints
                        .map(|constraints| SizeConstraints {
                            width: Constraint::new(
                                constraints
                                    .width
                                    .lower
                                    .map(|lower| lower + padding.leading + padding.trailing),
                                constraints
                                    .width
                                    .upper
                                    .map(|upper| upper + padding.leading + padding.trailing),
                            ),
                            height: Constraint::new(
                                constraints
                                    .height
                                    .lower
                                    .map(|lower| lower + padding.top + padding.bottom),
                                constraints
                                    .height
                                    .upper
                                    .map(|upper| upper + padding.top + padding.bottom),
                            ),
                            ..Default::default()
                        })
                }))
            }
            NodeType::Dynamic { .. } => {
                //
                self.tree
                    .get_node(node_id)
                    .children
                    .first()
                    .map(|child| {
                        self_constraints
                            .combine_parent_child(self.tree.get_node(*child).calculated_constraints)
                    })
                    .unwrap_or_default()
            }
            NodeType::Coupled {
                element: child_id, ..
            } => self_constraints.combine_parent_child(
                self.tree
                    .get_node(self.tree.get_node(node_id).children()[*child_id])
                    .calculated_constraints,
            ),
            NodeType::Intermediate { .. } => self
                .tree
                .get_node(node_id)
                .children()
                .first()
                .map(|child| {
                    self_constraints
                        .combine_parent_child(self.tree.get_node(*child).calculated_constraints)
                })
                .unwrap_or_default(),
            _ => self_constraints,
        }
    }

    fn allocate_node_area(&mut self, from: NodeId, state: &mut T, ui_state: &mut U) {
        let Some(available_area) = self.tree.get_node(from).area else {
            return;
        };

        match &self.tree.get_node(from).node_type {
            NodeType::Column {
                spacing,
                x_align,
                y_align,
            } => {
                self.layout_axis(
                    &self.tree.get_node(from).children.clone(),
                    *spacing,
                    available_area,
                    true,
                    *x_align,
                    *y_align,
                    state,
                    ui_state,
                );
            }
            NodeType::Row {
                spacing,
                y_align,
                x_align,
            } => {
                self.layout_axis(
                    &self.tree.get_node(from).children.clone(),
                    *spacing,
                    available_area,
                    false,
                    *x_align,
                    *y_align,
                    state,
                    ui_state,
                );
            }
            NodeType::Stack { x_align, y_align } => {
                let x_align = *x_align;
                let y_align = *y_align;
                for child_id in &self.tree.get_node(from).children.clone() {
                    let child_constraints = &self.tree.get_node(*child_id).calculated_constraints;
                    self.tree.get_node_mut(*child_id).area =
                        Some(available_area.constrained(child_constraints, x_align, y_align));
                }
            }
            NodeType::Padding(padding) => {
                if let Some(&child_id) = self.tree.get_node(from).children.first() {
                    let constrained_area = available_area.constrained(
                        &self.tree.get_node(from).calculated_constraints,
                        None,
                        None,
                    );

                    let child_area = Area {
                        x: constrained_area.x + padding.leading,
                        y: constrained_area.y + padding.top,
                        width: (constrained_area.width - padding.leading - padding.trailing)
                            .max(0.0),
                        height: (constrained_area.height - padding.top - padding.bottom).max(0.0),
                    };

                    self.tree.get_node_mut(child_id).area = Some(child_area);
                }
            }
            NodeType::Offset { x, y } => {
                if let Some(&child_id) = self.tree.get_node(from).children.first() {
                    let child_area = Area {
                        x: available_area.x + x,
                        y: available_area.y + y,
                        width: available_area.width,
                        height: available_area.height,
                    };
                    self.tree.get_node_mut(child_id).area = Some(child_area);
                }
            }
            NodeType::Coupled {
                over: _,
                element,
                coupled,
            } => {
                if self.tree.get_node(from).children.len() >= 2 {
                    let element_id = self.tree.get_node(from).children[*element];
                    let coupled_id = self.tree.get_node(from).children[*coupled];

                    let constrained_area = available_area.constrained(
                        &self.tree.get_node(element_id).calculated_constraints,
                        None,
                        None,
                    );

                    self.tree.get_node_mut(element_id).area = Some(constrained_area);
                    self.tree.get_node_mut(coupled_id).area = Some(constrained_area);
                }
            }
            _ => {
                let final_area = available_area.constrained(
                    &self.tree.get_node(from).calculated_constraints,
                    None,
                    None,
                );
                self.tree.get_node_mut(from).area = Some(final_area);

                let children = self.tree.get_node(from).children.clone();
                for &child_id in &children {
                    self.tree.get_node_mut(child_id).area = Some(final_area);
                }
            }
        }
    }

    fn layout_axis(
        &mut self,
        children: &[NodeId],
        spacing: f32,
        available_area: Area,
        is_vertical: bool,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
        state: &mut T,
        ui_state: &mut U,
    ) {
        if children.is_empty() {
            return;
        }

        let element_count = children.len();

        let filtered_element_count = element_count;
        let total_spacing = spacing * (element_count as i32 - 1).max(0) as f32;
        let available_size = if is_vertical {
            available_area.height
        } else {
            available_area.width
        } - total_spacing;

        let default_size = available_size / filtered_element_count as f32;

        let mut pool = 0.0;
        let mut final_sizes = vec![None; element_count];
        let mut room_to_grow = vec![0.0; element_count];
        let mut room_to_shrink = vec![0.0; element_count];

        for (i, &child_id) in children.iter().enumerate() {
            let constraints = &self.tree.get_node(child_id).constraints;
            let mut lower = if is_vertical {
                constraints
                    .dynamic_height
                    .as_ref()
                    .map(|f| f(available_area.width, state, ui_state))
                    .or(constraints.height_min)
            } else {
                constraints
                    .dynamic_width
                    .as_ref()
                    .map(|f| f(available_area.height, state, ui_state))
                    .or(constraints.width_min)
            };
            let mut upper = if is_vertical {
                constraints
                    .dynamic_height
                    .as_ref()
                    .map(|f| f(available_area.width, state, ui_state))
                    .or(constraints.height_max)
            } else {
                constraints
                    .dynamic_width
                    .as_ref()
                    .map(|f| f(available_area.height, state, ui_state))
                    .or(constraints.width_max)
            };

            if lower.is_none() && upper.is_none() {
                let child_constraints = &self.tree.get_node(child_id).constraints;

                let is_expanded = if let Some(size_constraints) =
                    self.tree.get_node(child_id).calculated_constraints
                {
                    if is_vertical {
                        child_constraints.expand_y || size_constraints.should_expand_y()
                    } else {
                        child_constraints.expand_x || size_constraints.should_expand_x()
                    }
                } else if is_vertical {
                    child_constraints.expand_y || child_constraints.height_max.is_none()
                } else {
                    child_constraints.expand_x || child_constraints.width_max.is_none()
                };

                if !is_expanded
                    && let Some(size_constraints) =
                        self.tree.get_node(child_id).calculated_constraints
                {
                    let intrinsic_size = if is_vertical {
                        size_constraints.height.lower.unwrap_or(0.0)
                    } else {
                        size_constraints.width.lower.unwrap_or(0.0)
                    };

                    if intrinsic_size > 0.0 {
                        lower = Some(intrinsic_size);
                        upper = Some(intrinsic_size);
                    }
                }
            }

            let mut final_size = None;

            if let Some(lower) = lower
                && default_size < lower
            {
                pool += default_size - lower;
                final_size = Some(lower);
            }
            if let Some(upper) = upper
                && default_size > upper
            {
                pool += default_size - upper;
                final_size = Some(upper);
            }

            if let Some(lower) = lower {
                if default_size >= lower {
                    room_to_shrink[i] = -(final_size.unwrap_or(default_size) - lower);
                }
            } else {
                room_to_shrink[i] = -default_size;
            }

            if let Some(upper) = upper {
                if default_size <= upper {
                    room_to_grow[i] = -(final_size.unwrap_or(default_size) - upper);
                }
            } else {
                room_to_grow[i] = default_size * 10.0;
            }

            final_sizes[i] = Some(final_size.unwrap_or(default_size));
        }

        fn can_accommodate(room: &[f32]) -> bool {
            room.iter().filter(|r| r.abs() > 0.).count() as f32 > 0.
        }

        let limit = 5;
        let mut i = 0;
        loop {
            if i > limit {
                break;
            }
            i += 1;
            let pool_empty = pool.abs() < 0.1;
            if !pool_empty && pool.is_sign_positive() && can_accommodate(&room_to_grow) {
                let mut enumerated_room: Vec<(usize, f32)> = room_to_grow
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (i, *v))
                    .filter(|(_, v)| *v != 0.)
                    .collect();
                enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let distribution_candidates = room_to_grow
                    .iter()
                    .filter(|r| r.abs() > 0. && r.is_sign_positive())
                    .count() as f32;
                let distribution_amount =
                    (pool / distribution_candidates).min(enumerated_room.first().unwrap().1);
                pool -= distribution_amount * distribution_candidates;
                enumerated_room.iter().for_each(|&(i, _)| {
                    if room_to_grow[i].abs() > 0. && room_to_grow[i].is_sign_positive() {
                        room_to_grow[i] -= distribution_amount;
                        if let Some(size) = &mut final_sizes[i] {
                            *size += distribution_amount;
                        }
                    }
                });
            } else if !pool_empty && pool.is_sign_negative() && can_accommodate(&room_to_shrink) {
                let mut enumerated_room: Vec<(usize, f32)> = room_to_shrink
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (i, *v))
                    .filter(|(_, v)| *v != 0.)
                    .collect();
                enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
                let distribution_candidates = room_to_shrink
                    .iter()
                    .filter(|r| r.abs() > 0. && r.is_sign_negative())
                    .count() as f32;
                let distribution_amount =
                    (pool / distribution_candidates).max(enumerated_room.first().unwrap().1);
                pool -= distribution_amount * distribution_candidates;
                enumerated_room.iter().for_each(|&(i, _)| {
                    if room_to_shrink[i].abs() > 0. && room_to_shrink[i].is_sign_negative() {
                        room_to_shrink[i] -= distribution_amount;
                        if let Some(size) = &mut final_sizes[i] {
                            *size += distribution_amount;
                        }
                    }
                });
            } else {
                break;
            }
        }

        let mut current_pos = if is_vertical {
            match y_align.unwrap_or(YAlign::Center) {
                YAlign::Top => available_area.y,
                YAlign::Center => available_area.y + (pool * 0.5),
                YAlign::Bottom => available_area.y + pool,
            }
        } else {
            match x_align.unwrap_or(XAlign::Center) {
                XAlign::Leading => available_area.x,
                XAlign::Center => available_area.x + (pool * 0.5),
                XAlign::Trailing => available_area.x + pool,
            }
        };

        for (i, &child_id) in children.iter().enumerate() {
            let child_size = final_sizes[i].unwrap_or(if filtered_element_count > 1 {
                0.0
            } else if is_vertical {
                available_area.height
            } else {
                available_area.width
            });

            let final_area = if is_vertical {
                Area {
                    x: available_area.x,
                    y: current_pos,
                    width: available_area.width,
                    height: child_size,
                }
            } else {
                Area {
                    x: current_pos,
                    y: available_area.y,
                    width: child_size,
                    height: available_area.height,
                }
            }
            .constrained(
                &self.tree.get_node(child_id).calculated_constraints,
                x_align,
                y_align,
            );

            self.tree.get_node_mut(child_id).area = Some(final_area);

            current_pos += child_size + spacing;
        }
    }

    fn draw_iterative(&mut self, state: &mut T, ui_state: &mut U) {
        let order: Vec<_> = self.tree.top_down_depth(self.tree.root_id).collect();
        let mut intermediate_stack: Vec<(usize, NodeId)> = Vec::new();
        for (depth, node_id) in order {
            let area = self.tree.get_node(node_id).area;
            if let Some((last_intermediate_depth, last_intermediate_id)) = intermediate_stack.last()
                && depth <= *last_intermediate_depth
                && let NodeType::Intermediate { after, .. } =
                    &self.tree.get_node(*last_intermediate_id).node_type
            {
                after(state, ui_state);
                intermediate_stack.pop();
            }
            match &self.tree.get_node(node_id).node_type {
                NodeType::Intermediate { before, .. } => {
                    if let Some(area) = area {
                        before(area, state, ui_state);
                    }
                    intermediate_stack.push((depth, node_id));
                }
                NodeType::Draw(draw_fn) => {
                    if let Some(area) = self.tree.get_node(node_id).area {
                        draw_fn(area, state, ui_state);
                    } else {
                        eprintln!("Unexpected draw node without area {:?}", self);
                    }
                }
                _ => {}
            }
        }
        while let Some((_last_intermediate_depth, last_intermediate_id)) = intermediate_stack.last()
            && let NodeType::Intermediate { after, .. } =
                &self.tree.get_node(*last_intermediate_id).node_type
        {
            after(state, ui_state);
            intermediate_stack.pop();
        }
    }
}

pub fn draw<T, U>(draw_fn: impl Fn(Area, &mut T, &mut U) + 'static) -> Node<T, U> {
    Node::new(NodeType::Draw(Box::new(draw_fn)))
}

pub fn column<T, U>(elements: Vec<Node<T, U>>) -> Node<T, U> {
    Node::new(NodeType::Column {
        spacing: 0.0,
        x_align: None,
        y_align: None,
    })
    .with_children(elements)
}

pub fn column_spaced<T, U>(spacing: f32, elements: Vec<Node<T, U>>) -> Node<T, U> {
    Node::new(NodeType::Column {
        spacing,
        x_align: None,
        y_align: None,
    })
    .with_children(elements)
}

pub fn column_aligned<T, U>(align: Align, elements: Vec<Node<T, U>>) -> Node<T, U> {
    let (x_align, y_align) = align.axis_aligns();
    Node::new(NodeType::Column {
        spacing: 0.0,
        x_align,
        y_align,
    })
    .with_children(elements)
}

pub fn column_spaced_aligned<T, U>(
    spacing: f32,
    align: Align,
    elements: Vec<Node<T, U>>,
) -> Node<T, U> {
    let (x_align, y_align) = align.axis_aligns();
    Node::new(NodeType::Column {
        spacing,
        x_align,
        y_align,
    })
    .with_children(elements)
}

pub fn row<T, U>(elements: Vec<Node<T, U>>) -> Node<T, U> {
    Node::new(NodeType::Row {
        spacing: 0.0,
        y_align: None,
        x_align: None,
    })
    .with_children(elements)
}

pub fn row_spaced<T, U>(spacing: f32, elements: Vec<Node<T, U>>) -> Node<T, U> {
    Node::new(NodeType::Row {
        spacing,
        x_align: None,
        y_align: None,
    })
    .with_children(elements)
}

pub fn row_aligned<T, U>(align: Align, elements: Vec<Node<T, U>>) -> Node<T, U> {
    let (x_align, y_align) = align.axis_aligns();
    Node::new(NodeType::Row {
        spacing: 0.,
        x_align,
        y_align,
    })
    .with_children(elements)
}

pub fn row_spaced_aligned<T, U>(
    spacing: f32,
    align: Align,
    elements: Vec<Node<T, U>>,
) -> Node<T, U> {
    let (x_align, y_align) = align.axis_aligns();
    Node::new(NodeType::Row {
        spacing,
        x_align,
        y_align,
    })
    .with_children(elements)
}

pub fn stack<T, U>(elements: Vec<Node<T, U>>) -> Node<T, U> {
    Node::new(NodeType::Stack {
        x_align: None,
        y_align: None,
    })
    .with_children(elements)
}

pub fn stack_aligned<T, U>(align: Align, elements: Vec<Node<T, U>>) -> Node<T, U> {
    let (x_align, y_align) = align.axis_aligns();
    Node::new(NodeType::Stack { x_align, y_align }).with_children(elements)
}

pub fn space<T, U>() -> Node<T, U> {
    Node::new(NodeType::Space)
}

pub fn empty<T, U>() -> Node<T, U> {
    Node::new(NodeType::Empty)
}

pub fn dynamic<T, U>(func: impl Fn(&mut T, &mut U) -> Node<T, U> + 'static) -> Node<T, U> {
    Node::new(NodeType::Dynamic {
        func: Box::new(func),
        expanded: false,
    })
}

pub fn area_reader<T, U>(
    func: impl Fn(Area, &mut T, &mut U) -> Node<T, U> + 'static,
) -> Node<T, U> {
    Node::new(NodeType::AreaReader {
        func: Box::new(func),
        expanded: false,
    })
}

pub fn intermediate<T, U>(
    before: impl Fn(Area, &mut T, &mut U) + 'static,
    after: impl Fn(&mut T, &mut U) + 'static,
    element: Node<T, U>,
) -> Node<T, U> {
    Node::new(NodeType::Intermediate {
        before: Box::new(before),
        after: Box::new(after),
    })
    .with_children(vec![element])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanded_constrained() {
        Layout::new(
            //
            dynamic(|_, _| draw(|_, _, _: &mut ()| {}))
                .height(30.)
                .width(30.),
        )
        .debug_visualize(Area::new(0.0, 0.0, 100.0, 100.0), &mut (), &mut ());
    }

    #[test]
    fn test_intermediate_at_root() {
        let mut draw_tracker = 0;
        Layout::new(
            //
            intermediate(
                |a, t, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                    assert_eq!(*t, 0);
                    *t = -1;
                },
                |t, _| {
                    assert_eq!(*t, 1);
                    *t = 3;
                },
                draw(|_, t, _: &mut ()| {
                    assert_eq!(*t, -1);
                    *t = 1;
                }),
            ),
        )
        .draw(
            Area::new(0.0, 0.0, 100.0, 100.0),
            &mut draw_tracker,
            &mut (),
        );
        assert_eq!(draw_tracker, 3);
    }

    #[test]
    fn test_intermediates_nested() {
        let mut draw_tracker = 0;
        Layout::new(
            //
            intermediate(
                |a, t, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                    assert_eq!(*t, 0);
                    *t = -1;
                },
                |t, _| {
                    assert_eq!(*t, 4);
                    *t = 3;
                },
                intermediate(
                    |a, t, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 100.));
                        assert_eq!(*t, -1);
                        *t = -2;
                    },
                    |t, _| {
                        assert_eq!(*t, 2);
                        *t = 4;
                    },
                    draw(|_, t, _: &mut ()| {
                        assert_eq!(*t, -2);
                        *t = 2;
                    }),
                ),
            ),
        )
        .draw(
            Area::new(0.0, 0.0, 100.0, 100.0),
            &mut draw_tracker,
            &mut (),
        );
        assert_eq!(draw_tracker, 3);
    }

    #[test]
    fn test_expands_nested_nodes() {
        let mut draw_tracker = 0;
        Layout::new(
            //
            area_reader(|_, _, _| {
                dynamic(|_, _| {
                    area_reader(|_, _, _| {
                        dynamic(|_, _| {
                            draw(|_, t, _: &mut ()| {
                                *t += 1;
                            })
                        })
                    })
                })
            }),
        )
        .draw(
            Area::new(0.0, 0.0, 100.0, 100.0),
            &mut draw_tracker,
            &mut (),
        );
        assert_eq!(draw_tracker, 1);
    }

    #[test]
    fn test_draws_all_expanded_nodes() {
        let mut draw_tracker = 0;
        Layout::new(
            //
            area_reader(|_, _, _| {
                stack(vec![
                    dynamic(|_, _| {
                        stack(vec![
                            area_reader(|_, _, _| {
                                draw(|_, t, _: &mut ()| {
                                    *t += 1;
                                })
                            }),
                            draw(|_, t, _: &mut ()| {
                                *t += 1;
                            }),
                        ])
                    }),
                    draw(|_, t, _: &mut ()| {
                        *t += 1;
                    }),
                ])
            }),
        )
        .draw(
            Area::new(0.0, 0.0, 100.0, 100.0),
            &mut draw_tracker,
            &mut (),
        );
        assert_eq!(draw_tracker, 3);
    }

    #[test]
    fn test_simple_column_layout() {
        let layout_node = column(vec![
            draw(|area, _: &mut (), _: &mut ()| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 50.0);
            })
            .height(50.0),
            draw(|area, _: &mut (), _: &mut ()| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 50.0);
                assert_eq!(area.y, 50.0);
            })
            .height(50.0),
        ]);

        let mut mvp_layout = Layout::new(layout_node);
        let bounds = Area::new(0.0, 0.0, 100.0, 100.0);
        mvp_layout.draw(bounds, &mut (), &mut ());
    }

    #[test]
    fn test_simple_row_layout() {
        let layout_node = row(vec![
            draw(|area, _: &mut (), _: &mut ()| {
                assert_eq!(area.width, 50.0);
                assert_eq!(area.height, 100.0);
            })
            .width(50.0),
            draw(|area, _: &mut (), _: &mut ()| {
                assert_eq!(area.width, 50.0);
                assert_eq!(area.height, 100.0);
                assert_eq!(area.x, 50.0);
            })
            .width(50.0),
        ]);

        let mut mvp_layout = Layout::new(layout_node);
        let bounds = Area::new(0.0, 0.0, 100.0, 100.0);
        mvp_layout.draw(bounds, &mut (), &mut ());
    }

    #[test]
    fn test_nested_layout() {
        let layout_node = column(vec![
            row(vec![
                draw(|area, _: &mut (), _: &mut ()| {
                    assert_eq!(area.width, 50.0);
                    assert_eq!(area.height, 25.0);
                })
                .width(50.0),
                draw(|area, _: &mut (), _: &mut ()| {
                    assert_eq!(area.width, 50.0);
                    assert_eq!(area.height, 25.0);
                    assert_eq!(area.x, 50.0);
                })
                .width(50.0),
            ])
            .height(25.0),
            draw(|area, _: &mut (), _: &mut ()| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 75.0);
                assert_eq!(area.y, 25.0);
            })
            .height(75.0),
        ]);

        let mut mvp_layout = Layout::new(layout_node);
        let bounds = Area::new(0.0, 0.0, 100.0, 100.0);
        mvp_layout.draw(bounds, &mut (), &mut ());
    }

    #[test]
    fn test_padding() {
        let layout_node = draw(|area, _: &mut (), _: &mut ()| {
            assert_eq!(area.x, 10.0);
            assert_eq!(area.y, 10.0);
            assert_eq!(area.width, 80.0);
            assert_eq!(area.height, 80.0);
        })
        .pad(10.0);

        let mut mvp_layout = Layout::new(layout_node);
        let bounds = Area::new(0.0, 0.0, 100.0, 100.0);
        mvp_layout.debug_visualize(bounds, &mut (), &mut ());
    }

    #[test]
    fn test_stack_layout() {
        let mut draw_count = 0;
        let layout_node = stack(vec![
            draw(|area, count: &mut i32, _: &mut ()| {
                *count += 1;
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 100.0);
            }),
            draw(|area, count: &mut i32, _: &mut ()| {
                *count += 1;
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 100.0);
            }),
        ]);

        let mut mvp_layout = Layout::new(layout_node);
        mvp_layout.draw(Area::new(0.0, 0.0, 100.0, 100.0), &mut draw_count, &mut ());
        assert_eq!(draw_count, 2);
    }

    #[test]
    fn test_dynamic_node() {
        let layout_node = dynamic(|state: &mut bool, _: &mut ()| {
            if *state {
                draw(|area, _, _| {
                    assert_eq!(area.width, 100.0);
                })
                .height(50.0)
            } else {
                draw(|area, _, _| {
                    assert_eq!(area.width, 100.0);
                })
                .height(25.0)
            }
        });

        let mut state = true;
        let mut mvp_layout = Layout::new(layout_node);
        mvp_layout.draw(Area::new(0.0, 0.0, 100.0, 100.0), &mut state, &mut ());
    }

    #[test]
    fn test_dynamic_node_drawing_issue() {
        struct TestState {
            counter: i32,
            draw_calls: Vec<String>,
        }

        let layout_node = dynamic(|state: &mut TestState, _: &mut ()| {
            state.counter += 1;

            column(vec![
                draw(|_, state: &mut TestState, _| {
                    state
                        .draw_calls
                        .push(format!("dynamic_child_{}", state.counter));
                })
                .height(20.0),
                draw(|_, state: &mut TestState, _| {
                    state.draw_calls.push("static_draw".to_string());
                })
                .height(30.0),
            ])
        });

        let mut state = TestState {
            counter: 0,
            draw_calls: Vec::new(),
        };
        let mut mvp_layout = Layout::new(layout_node);
        mvp_layout.draw(Area::new(0.0, 0.0, 100.0, 100.0), &mut state, &mut ());

        println!("Draw calls: {:?}", state.draw_calls);
        assert!(
            !state.draw_calls.is_empty(),
            "Dynamic node children should have been drawn"
        );
        assert!(
            state.draw_calls.contains(&"static_draw".to_string()),
            "Static draw should be called"
        );
    }

    #[test]
    fn test_nested_dynamic_nodes() {
        struct TestState {
            inner_counter: i32,
            draw_calls: Vec<String>,
        }

        let layout_node = dynamic(|_state: &mut TestState, _: &mut ()| {
            column(vec![
                draw(|_, state: &mut TestState, _| {
                    state.draw_calls.push("outer_before".to_string());
                })
                .height(10.0),
                dynamic(|state: &mut TestState, _: &mut ()| {
                    state.inner_counter += 1;
                    draw(|_, state: &mut TestState, _| {
                        state
                            .draw_calls
                            .push(format!("inner_{}", state.inner_counter));
                    })
                    .height(20.0)
                }),
                draw(|_, state: &mut TestState, _| {
                    state.draw_calls.push("outer_after".to_string());
                })
                .height(15.0),
            ])
        });

        let mut state = TestState {
            inner_counter: 0,
            draw_calls: Vec::new(),
        };
        let mut mvp_layout = Layout::new(layout_node);
        mvp_layout.draw(Area::new(0.0, 0.0, 100.0, 100.0), &mut state, &mut ());

        println!("Nested draw calls: {:?}", state.draw_calls);
        assert!(
            state.draw_calls.contains(&"outer_before".to_string()),
            "Outer before should be drawn"
        );
        assert!(
            state.draw_calls.contains(&"outer_after".to_string()),
            "Outer after should be drawn"
        );
        assert!(
            state
                .draw_calls
                .iter()
                .any(|call| call.starts_with("inner_")),
            "Inner dynamic should be drawn"
        );
    }

    #[test]
    fn test_row_dynamic() {
        Layout::new({
            column(vec![
                row(vec![
                    space().height(0.),
                    dynamic(|_, _| {
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(50., 0., 50., 20.));
                        })
                        .dynamic_height(|_, _, _| 20.)
                    }),
                ]),
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 20., 100., 80.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    }

    #[test]
    fn test_static_vs_dynamic_height() {
        let mut layout1 = Layout::new({
            column(vec![
                row(vec![
                    space().height(0.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 50., 30.));
                    })
                    .height(30.),
                ]),
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 30., 100., 70.));
                }),
            ])
        });
        layout1.draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());

        let mut layout2 = Layout::new({
            column(vec![
                row(vec![
                    space().height(0.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 50., 30.));
                    })
                    .dynamic_height(|_, _, _| 30.),
                ]),
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 30., 100., 70.));
                }),
            ])
        });
        layout2.draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    }

    #[cfg(test)]
    mod layout_tests {

        use super::*;
        #[test]
        fn test_seq_align_on_axis() {
            Layout::new({
                row_aligned(
                    Align::Leading,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 10., 100.));
                        })
                        .width(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(10., 0., 30., 100.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(30., 0., 10., 100.));
                    })
                    .width(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(40., 0., 30., 100.));
                    })
                    .width(30.),
                ])
                .align(Align::CenterX)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row_aligned(
                    Align::Trailing,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(60., 0., 10., 100.));
                        })
                        .width(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(70., 0., 30., 100.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_aligned(
                    Align::Top,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 100., 10.));
                        })
                        .height(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 10., 100., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 30., 100., 10.));
                    })
                    .height(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 40., 100., 30.));
                    })
                    .height(30.),
                ])
                .align(Align::CenterY)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_aligned(
                    Align::Bottom,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 60., 100., 10.));
                        })
                        .height(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 70., 100., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_seq_align_off_axis() {
            Layout::new({
                column_aligned(
                    Align::Leading,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 10., 50.));
                        })
                        .width(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 50., 30., 50.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(45., 0., 10., 50.));
                    })
                    .width(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(35., 50., 30., 50.));
                    })
                    .width(30.),
                ])
                .align(Align::CenterX)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_aligned(
                    Align::Trailing,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(90., 0., 10., 50.));
                        })
                        .width(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(70., 50., 30., 50.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row_aligned(
                    Align::Top,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 50., 10.));
                        })
                        .height(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(50., 0., 50., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 45., 50., 10.));
                    })
                    .height(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 35., 50., 30.));
                    })
                    .height(30.),
                ])
                .align(Align::CenterY)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row_aligned(
                    Align::Bottom,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 90., 50., 10.));
                        })
                        .height(10.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(50., 70., 50., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_seq_align_on_axis_nested_seq() {
            Layout::new({
                row_aligned(
                    Align::Leading,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(0., 0., 10., 100.));
                            })
                            .width(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(10., 0., 30., 100.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    row(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(30., 0., 10., 100.));
                        })
                        .width(10.),
                    ]),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(40., 0., 30., 100.));
                    })
                    .width(30.),
                ])
                .align(Align::CenterX)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row_aligned(
                    Align::Trailing,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(60., 0., 10., 100.));
                            })
                            .width(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(70., 0., 30., 100.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_aligned(
                    Align::Top,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(0., 0., 100., 10.));
                            })
                            .height(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 10., 100., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    row(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 30., 100., 10.));
                        })
                        .height(10.),
                    ]),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 40., 100., 30.));
                    })
                    .height(30.),
                ])
                .align(Align::CenterY)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_aligned(
                    Align::Bottom,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(0., 60., 100., 10.));
                            })
                            .height(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 70., 100., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_seq_align_off_axis_nested_seq() {
            Layout::new({
                column_aligned(
                    Align::Leading,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(0., 0., 10., 50.));
                            })
                            .width(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 50., 30., 50.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    row(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(45., 0., 10., 50.));
                        })
                        .width(10.),
                    ]),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(35., 50., 30., 50.));
                    })
                    .width(30.),
                ])
                .align(Align::CenterX)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_aligned(
                    Align::Trailing,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(90., 0., 10., 50.));
                            })
                            .width(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(70., 50., 30., 50.));
                        })
                        .width(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row_aligned(
                    Align::Top,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(0., 0., 50., 10.));
                            })
                            .height(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(50., 0., 50., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    row(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 45., 50., 10.));
                        })
                        .height(10.),
                    ]),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 35., 50., 30.));
                    })
                    .height(30.),
                ])
                .align(Align::CenterY)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row_aligned(
                    Align::Bottom,
                    vec![
                        row(vec![
                            draw(|a, _, _| {
                                assert_eq!(a, Area::new(0., 90., 50., 10.));
                            })
                            .height(10.),
                        ]),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(50., 70., 50., 30.));
                        })
                        .height(30.),
                    ],
                )
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_aspect_ratio() {
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                })
                .aspect_width(1.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(25., 0., 50., 100.));
                })
                .aspect_width(0.5)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 100.));
                })
                .aspect_width(0.5)
                .align(Align::Leading)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 100.));
                })
                .aspect_width(0.5)
                .align(Align::Trailing)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());

            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 25., 100., 50.));
                })
                .aspect_height(2.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 50.));
                })
                .aspect_height(2.)
                .align(Align::Top)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 50., 100., 50.));
                })
                .aspect_height(2.)
                .align(Align::Bottom)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_aspect_ratio_in_seq() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 100.));
                    })
                    .aspect_width(1.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 0., 50., 100.));
                    })
                    .aspect_width(0.5),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 50., 100.));
                    })
                    .aspect_width(0.5)
                    .align(Align::Leading),
                ])
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 50., 100.));
                    })
                    .aspect_width(0.5)
                    .align(Align::Trailing),
                ])
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_aspect_ratio_nested() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 200., 50.));
                    }),
                    row(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 50., 150., 50.));
                        }),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(150., 50., 50., 50.));
                        })
                        .aspect_width(1.),
                    ]),
                ])
            })
            .debug_visualize(Area::new(0., 0., 200., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_pad() {
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(10., 10., 80., 80.));
                })
                .pad(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(10., 0., 80., 100.));
                })
                .pad_x(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 80.));
                })
                .pad_y(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(10., 0., 90., 100.));
                })
                .pad_leading(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 90., 100.));
                })
                .pad_trailing(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 90.));
                })
                .pad_top(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 90.));
                })
                .pad_bottom(10.)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_aspect_ratio_in_pad() {
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(25., 0., 50., 100.));
                })
                .aspect_width(0.5)
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(30., 10., 40., 80.));
                    })
                    .aspect_width(0.5)
                    .pad(10.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(35., 10., 30., 80.));
                    })
                    .pad(10.)
                    .aspect_width(0.5),
                ])
            })
            .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_aspect_ratio_fit() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 50.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 50., 50., 50.));
                    })
                    .aspect_width(1.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 0., 50., 50.));
                    })
                    .aspect_width(1.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 50., 50., 50.));
                    })
                    .aspect_width(1.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 50., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 25., 50., 50.));
                    })
                    .aspect_height(1.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 25., 50., 50.));
                    })
                    .aspect_height(1.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 25., 50., 50.));
                    })
                    .aspect_height(1.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_space_expansion() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 1., 100.));
                    })
                    .width(1.),
                    space(),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(998., 0., 1., 100.));
                    })
                    .width(1.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(999., 0., 1., 100.));
                    })
                    .width(1.),
                ])
            })
            .draw(Area::new(0., 0., 1000., 100.), &mut (), &mut ());
        }
        // #[test]
        // fn test_explicit_aspect() {
        //     Layout::new({
        //         column_spaced(
        //             10.,
        //             vec![
        //                 draw(|a, _, _| {
        //                     assert_eq!(a, Area::new(45., 0., 10., 20.));
        //                 })
        //                 .width(10.)
        //                 .aspect_width(0.5),
        //                 draw(|a, _, _| {
        //                     // assert_eq!(a, Area::new(0., 30., 100., 70.));
        //                 }),
        //             ],
        //         )
        //     })
        //     .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        // }
        #[test]
        fn test_explicit_with_padding() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(10., 10., 80., 20.));
                    })
                    .height(20.)
                    .pad(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 40., 100., 60.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_explicit_in_explicit() {
            Layout::new({
                draw(|a, _, _| {
                    assert_eq!(a, Area::new(40., 0., 20., 100.));
                })
                .width_range(20.0..)
                .pad(0.)
                .attach_under(draw(|a, _, _| {
                    assert_eq!(a, Area::new(40., 0., 20., 100.));
                }))
                .width_range(..10.)
                .attach_under(draw(|a, _, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 100.));
                }))
            })
            .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_compressed_expanded_respects_lower_bound() {
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., -50., 100., 200.));
                    })
                    .height(200.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., -50., 100., 200.));
                    }),
                ])
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    stack(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., -50., 100., 200.));
                        })
                        .height(200.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., -50., 100., 200.));
                        }),
                    ])
                    .expand(),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_compressed_aspect_ratio() {
            Layout::<(), ()>::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 25., 50., 50.));
                    })
                    .aspect_width(1.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 50., 100.));
                    })
                    .width(50.),
                ])
                .attach_under(draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }))
            })
            .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_dynamic_attached() {
            Layout::new({
                row(vec![
                    space(),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 25., 25., 50.));
                    })
                    .dynamic_height(|h, _, _| h * 2.)
                    .attach_under(draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 25., 25., 50.));
                    })),
                    space(),
                    space(),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
    }

    #[cfg(test)]
    mod sequence_tests {
        use super::*;
        #[test]
        fn test_column_basic() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 50.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 50., 100., 50.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_column_constrained_1() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 10.));
                    })
                    .height(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 10., 100., 90.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 10.));
                    })
                    .height(10.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 10., 100., 90.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_column_constrained_2() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 90.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 90., 100., 10.));
                    })
                    .height(10.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 90.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 90., 100., 10.));
                    })
                    .height(10.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_row_basic() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 50., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 50., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_row_constrained_1() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 25., 10., 50.));
                    })
                    .width(10.)
                    .height(50.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(10., 0., 90., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::Top),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(10., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(20., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::Bottom),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(30., 0., 70., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_row_constrained_2() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 70., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(70., 0., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::Top),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(80., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(90., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::Bottom),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 70., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(70., 0., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::Top),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(80., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(90., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::Bottom),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_stack_basic() {
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }

        #[test]
        fn test_stack_alignment() {
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::TopLeading),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(45., 0., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::TopCenter),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(90., 0., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::TopTrailing),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(90., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::CenterTrailing),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(90., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::BottomTrailing),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(45., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::BottomCenter),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::BottomLeading),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::CenterLeading),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(45., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.)
                    .align(Align::CenterCenter),
                ])
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_sequence_spacing() {
            Layout::new({
                row_spaced(
                    10.,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 40., 10., 20.));
                        })
                        .width(10.)
                        .height(20.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(20., 0., 25., 100.));
                        }),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(55., 40., 10., 20.));
                        })
                        .width(10.)
                        .height(20.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(75., 0., 25., 100.));
                        }),
                    ],
                )
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column_spaced(
                    10.,
                    vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 100., 15.));
                        }),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(45., 25., 10., 20.));
                        })
                        .width(10.)
                        .height(20.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 55., 100., 15.));
                        }),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(45., 80., 10., 20.));
                        })
                        .width(10.)
                        .height(20.),
                    ],
                )
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
        #[test]
        fn test_row_with_constrained_item() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 30., 100.));
                    })
                    .width(30.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(30., 0., 70., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }

        #[test]
        fn test_nested_row_with_constrained_item() {
            Layout::new({
                row(vec![
                    row(vec![
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 20., 100.));
                        })
                        .width(20.),
                        draw(|a, _, _| {
                            assert_eq!(a, Area::new(20., 0., 30., 100.));
                        }),
                    ])
                    .width(50.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 50., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }

        #[test]
        fn test_stack_with_constrained_item() {
            Layout::new({
                stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(25., 25., 50., 50.));
                    })
                    .width(50.)
                    .height(50.),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }

        #[test]
        fn test_row_with_multiple_constrained_items() {
            Layout::new({
                row(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 0., 20., 100.));
                    })
                    .width(20.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(20., 0., 30., 100.));
                    })
                    .width(30.),
                    draw(|a, _, _| {
                        assert!((a.x - 50.0).abs() < 0.001);
                        assert!((a.y - 0.0).abs() < 0.001);
                        assert!((a.width - 50.0).abs() < 0.001);
                        assert!((a.height - 100.0).abs() < 0.001);
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }

        #[test]
        fn test_row_with_constrained_height_in_column() {
            Layout::new({
                column(vec![
                    draw(|a, _, _| {
                        // Should get 40px height (half of remaining 80px after row takes 20px)
                        assert_eq!(a, Area::new(0., 0., 100., 40.));
                    }),
                    row(vec![
                        dynamic(|_, _| {
                            draw(|a, _, _| {
                                // Row content should be 20px tall
                                assert_eq!(a, Area::new(0., 40., 50., 20.));
                            })
                            .height(20.)
                        }),
                        dynamic(|_, _| {
                            draw(|a, _, _| {
                                // Row content should be 20px tall
                                assert_eq!(a, Area::new(50., 40., 50., 20.));
                            })
                            .height(20.)
                        }),
                    ]),
                    draw(|a, _, _| {
                        // Should get 40px height (half of remaining 80px after row takes 20px)
                        assert_eq!(a, Area::new(0., 60., 100., 40.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
    }
}
