use std::collections::{HashMap, VecDeque};
use std::ops::RangeBounds;
use std::rc::Rc;

type NodeId = usize;
type AreaReaderFn<T, U> = Box<dyn Fn(Area, &mut T, &mut U) -> Node<T, U>>;
type DynamicNodeFn<T, U> = Box<dyn Fn(&mut T, &mut U) -> Node<T, U>>;
type DrawableFn<T, U> = Box<dyn Fn(Area, &mut T, &mut U)>;
type DimensionFn<T, U> = Option<Rc<dyn Fn(f32, &mut T, &mut U) -> f32>>;

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

    pub(crate) fn combine_explicit_with_child(self, child: Self) -> Self {
        Constraint::new(
            self.lower
                .or(child.lower.map(|cl| cl.min(self.upper.unwrap_or(cl)))),
            self.upper
                .or(child.upper.map(|cl| cl.max(self.lower.unwrap_or(cl)))),
        )
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
}

impl SizeConstraints {
    pub(crate) fn should_expand_x(&self) -> bool {
        self.expand_x || self.width.upper.is_none()
    }

    pub(crate) fn should_expand_y(&self) -> bool {
        self.expand_y || self.height.upper.is_none()
    }
}

impl Default for SizeConstraints {
    fn default() -> Self {
        SizeConstraints {
            width: Constraint::none(),
            height: Constraint::none(),
            expand_x: false,
            expand_y: false,
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
    AreaReader(AreaReaderFn<T, U>),
    Dynamic {
        func: DynamicNodeFn<T, U>,
        computed: Option<NodeId>,
    },
    Visibility {
        visible: bool,
    },
    Coupled {
        over: bool,
        element: NodeId,
        coupled: NodeId,
    },
    Explicit,
}

struct NodeData<T, U> {
    pub(crate) node_type: NodeType<T, U>,
    pub(crate) constraints: NodeConstraints<T, U>,
    pub children: Vec<NodeId>,
    pub area: Area,
    pub content_hash: u64,
}

pub struct Node<T, U> {
    pub(crate) node_type: NodeType<T, U>,
    pub(crate) constraints: NodeConstraints<T, U>,
    pub children: Vec<Node<T, U>>,
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

    pub(crate) fn with_width_constraints(
        mut self,
        width_min: Option<f32>,
        width_max: Option<f32>,
    ) -> Self {
        self.constraints.width_min = width_min;
        self.constraints.width_max = width_max;
        self
    }

    pub(crate) fn with_height_constraints(
        mut self,
        height_min: Option<f32>,
        height_max: Option<f32>,
    ) -> Self {
        self.constraints.height_min = height_min;
        self.constraints.height_max = height_max;
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

    pub fn width_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (width_min, width_max) = Self::extract_bounds(range);
        Node::new(NodeType::Explicit)
            .with_width_constraints(width_min, width_max)
            .with_children(vec![self])
    }

    pub fn height_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (height_min, height_max) = Self::extract_bounds(range);
        Node::new(NodeType::Explicit)
            .with_height_constraints(height_min, height_max)
            .with_children(vec![self])
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

    pub fn visible(self, visible: bool) -> Node<T, U> {
        Node::new(NodeType::Visibility { visible }).with_children(vec![self])
    }

    pub fn attach_under(self, node: Node<T, U>) -> Node<T, U> {
        Node::new(NodeType::Coupled {
            over: false,
            element: 0,
            coupled: 1,
        })
        .with_children(vec![self, node])
    }

    pub fn attach_over(self, node: Node<T, U>) -> Node<T, U> {
        Node::new(NodeType::Coupled {
            over: true,
            element: 0,
            coupled: 1,
        })
        .with_children(vec![self, node])
    }

    pub fn dynamic_width(self, f: impl Fn(f32, &mut T, &mut U) -> f32 + 'static) -> Self {
        Node::new(NodeType::Explicit)
            .with_constraints(NodeConstraints {
                dynamic_width: Some(Rc::new(f)),
                ..Default::default()
            })
            .with_children(vec![self])
    }

    pub fn dynamic_height(self, f: impl Fn(f32, &mut T, &mut U) -> f32 + 'static) -> Self {
        Node::new(NodeType::Explicit)
            .with_constraints(NodeConstraints {
                dynamic_height: Some(Rc::new(f)),
                ..Default::default()
            })
            .with_children(vec![self])
    }

    fn with_constraints(mut self, constraints: NodeConstraints<T, U>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn aspect_width(self, ratio: f32) -> Self {
        self.dynamic_width(move |height, _, _| height * ratio)
    }

    pub fn aspect_height(self, ratio: f32) -> Self {
        self.dynamic_height(move |width, _, _| width / ratio)
    }
}

#[derive(Default)]
struct LayoutCache {
    constraint_results: HashMap<u64, SizeConstraints>,
}

pub struct Layout<T, U> {
    nodes: Vec<NodeData<T, U>>,
    cache: LayoutCache,
    work_queue: VecDeque<NodeId>,
    root_id: Option<NodeId>,
}

impl<T, U> Layout<T, U> {
    pub fn new(root_node: Node<T, U>) -> Self {
        let mut layout = Self {
            nodes: Vec::new(),
            cache: LayoutCache::default(),
            work_queue: VecDeque::new(),
            root_id: None,
        };
        layout.root_id = Some(layout.flatten_tree(root_node));
        layout
    }

    #[cfg(test)]
    pub fn debug_visualize(&mut self, bounds: Area, state: &mut T, ui_state: &mut U) {
        use std::cell::RefCell;
        use std::rc::Rc;

        fn find_all_draw_nodes<T, U>(
            nodes: &[NodeData<T, U>],
            root_id: Option<NodeId>,
            draw_nodes: &mut Vec<NodeId>,
        ) {
            fn find_recursive<T, U>(
                nodes: &[NodeData<T, U>],
                node_id: NodeId,
                draw_nodes: &mut Vec<NodeId>,
            ) {
                let node = &nodes[node_id];
                if matches!(node.node_type, NodeType::Draw(_)) {
                    draw_nodes.push(node_id);
                }
                for &child_id in &node.children {
                    find_recursive(nodes, child_id, draw_nodes);
                }
            }

            if let Some(node_id) = root_id {
                find_recursive(nodes, node_id, draw_nodes);
            }
        }

        fn replace_draw_nodes_with_debug<T, U>(
            nodes: &mut [NodeData<T, U>],
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

                nodes[node_id].node_type =
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
        find_all_draw_nodes(&self.nodes, self.root_id, &mut draw_node_indices);

        // Replace all Draw nodes with debug draw functions
        replace_draw_nodes_with_debug(
            &mut self.nodes,
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

    fn flatten_tree(&mut self, root_node: Node<T, U>) -> NodeId {
        struct WorkItem<T, U> {
            node: Node<T, U>,
            node_id: NodeId,
        }

        let root_id = self.nodes.len();
        let mut work_queue = std::collections::VecDeque::new();

        let mut parent_child_map = std::collections::HashMap::new();
        work_queue.push_back(WorkItem {
            node: root_node,
            node_id: root_id,
        });

        while let Some(work_item) = work_queue.pop_front() {
            self.nodes.push(NodeData {
                node_type: work_item.node.node_type,
                constraints: work_item.node.constraints,
                children: Vec::new(),
                area: Area::default(),
                content_hash: 0,
            });

            let mut child_ids = Vec::new();
            for child in work_item.node.children {
                let child_id = self.nodes.len() + work_queue.len();
                child_ids.push(child_id);
                work_queue.push_back(WorkItem {
                    node: child,
                    node_id: child_id,
                });
            }
            parent_child_map.insert(work_item.node_id, child_ids);
        }

        for (node_id, child_ids) in parent_child_map {
            self.nodes[node_id].children = child_ids;
        }

        root_id
    }

    pub fn draw(&mut self, available_area: Area, state: &mut T, ui_state: &mut U) {
        if let Some(root_id) = self.root_id {
            self.layout_iterative(root_id, available_area, state, ui_state);
            self.draw_iterative(root_id, state, ui_state, true);
        }
    }

    fn layout_iterative(
        &mut self,
        root_id: NodeId,
        available_area: Area,
        state: &mut T,
        ui_state: &mut U,
    ) {
        self.queue_constraints_pass(root_id);
        while let Some(node_id) = self.work_queue.pop_front() {
            self.resolve_node_constraints(node_id, state, ui_state);
        }

        self.nodes[root_id].area = available_area;
        self.queue_allocation_pass(root_id);
        while let Some(node_id) = self.work_queue.pop_front() {
            self.allocate_node_area(node_id, state, ui_state);
        }
    }

    fn queue_constraints_pass(&mut self, root_id: NodeId) {
        let mut stack = vec![(root_id, false)];
        let mut visit_order = Vec::new();

        while let Some((node_id, visited)) = stack.pop() {
            if visited {
                visit_order.push(node_id);
            } else {
                stack.push((node_id, true));

                for &child_id in &self.nodes[node_id].children.clone() {
                    stack.push((child_id, false));
                }
            }
        }

        for node_id in visit_order {
            self.work_queue.push_back(node_id);
        }
    }

    fn queue_allocation_pass(&mut self, root_id: NodeId) {
        let mut stack = vec![root_id];

        while let Some(node_id) = stack.pop() {
            self.work_queue.push_back(node_id);

            for &child_id in self.nodes[node_id].children.iter().rev() {
                stack.push(child_id);
            }
        }
    }

    fn resolve_node_constraints(&mut self, node_id: NodeId, state: &mut T, ui_state: &mut U) {
        let node_type_matches_dynamic =
            matches!(&self.nodes[node_id].node_type, NodeType::Dynamic { .. });
        if node_type_matches_dynamic {
            if let NodeType::Dynamic { func, computed } = &self.nodes[node_id].node_type {
                if computed.is_none() {
                    let dynamic_result = func(state, ui_state);
                    let computed_id = self.flatten_tree(dynamic_result);

                    if let NodeType::Dynamic { computed, .. } = &mut self.nodes[node_id].node_type {
                        *computed = Some(computed_id);
                    }
                    self.nodes[node_id].children = vec![computed_id];
                }
            }
        }

        let node_type_matches_area_reader =
            matches!(&self.nodes[node_id].node_type, NodeType::AreaReader(_));
        if node_type_matches_area_reader {
            if let NodeType::AreaReader(reader_fn) = &self.nodes[node_id].node_type {
                let current_area = self.nodes[node_id].area;
                let area_result = reader_fn(current_area, state, ui_state);
                let computed_id = self.flatten_tree(area_result);
                self.nodes[node_id].children = vec![computed_id];

                let computed_node_type =
                    std::mem::replace(&mut self.nodes[computed_id].node_type, NodeType::Empty);
                let computed_constraints = std::mem::take(&mut self.nodes[computed_id].constraints);
                self.nodes[node_id].node_type = computed_node_type;
                self.nodes[node_id].constraints = computed_constraints;
            }
        }

        let constraints = self.calculate_node_constraints(node_id, state, ui_state);

        let content_hash = self.calculate_content_hash(node_id);
        self.cache
            .constraint_results
            .insert(content_hash, constraints);
        self.nodes[node_id].content_hash = content_hash;
    }

    fn apply_intrinsic_constraints(
        &self,
        effective_constraints: &mut NodeConstraints<T, U>,
        size_constraints: &SizeConstraints,
        constraints: &NodeConstraints<T, U>,
        is_vertical: bool,
        hug_cross_in_column: bool,
        hug_main_in_column: bool,
    ) {
        let intrinsic_width = size_constraints.width.lower.unwrap_or(0.0);
        let intrinsic_height = size_constraints.height.lower.unwrap_or(0.0);

        if is_vertical {
            if hug_cross_in_column
                && !constraints.expand_x
                && !size_constraints.should_expand_x()
                && intrinsic_width > 0.0
            {
                effective_constraints.width_min = Some(intrinsic_width);
                effective_constraints.width_max = Some(intrinsic_width);
            }
            if hug_main_in_column
                && !constraints.expand_y
                && !size_constraints.should_expand_y()
                && intrinsic_height > 0.0
            {
                effective_constraints.height_min = Some(intrinsic_height);
                effective_constraints.height_max = Some(intrinsic_height);
            }
        } else {
            if !hug_cross_in_column
                && !constraints.expand_x
                && !size_constraints.should_expand_x()
                && intrinsic_width > 0.0
            {
                effective_constraints.width_min = Some(intrinsic_width);
                effective_constraints.width_max = Some(intrinsic_width);
            }
            if !hug_main_in_column
                && !constraints.expand_y
                && !size_constraints.should_expand_y()
                && intrinsic_height > 0.0
            {
                effective_constraints.height_min = Some(intrinsic_height);
                effective_constraints.height_max = Some(intrinsic_height);
            }
        }
    }

    fn combine_child_constraints(
        &self,
        node: &NodeData<T, U>,
        node_width: Constraint,
        node_height: Constraint,
        width_combiner: impl Fn(Constraint, Constraint, f32) -> Constraint,
        height_combiner: impl Fn(Constraint, Constraint, f32) -> Constraint,
        spacing: f32,
    ) -> SizeConstraints {
        let mut combined_width = Constraint::none();
        let mut combined_height = Constraint::none();
        let mut any_expand_x = false;
        let mut any_expand_y = false;

        for (i, &child_id) in node.children.iter().enumerate() {
            let child_hash = self.nodes[child_id].content_hash;
            if let Some(&child_constraints) = self.cache.constraint_results.get(&child_hash) {
                combined_width = if i == 0 {
                    child_constraints.width
                } else {
                    width_combiner(combined_width, child_constraints.width, spacing)
                };

                combined_height = if i == 0 {
                    child_constraints.height
                } else {
                    height_combiner(combined_height, child_constraints.height, spacing)
                };

                any_expand_x |= child_constraints.should_expand_x();
                any_expand_y |= child_constraints.should_expand_y();
            }
        }

        SizeConstraints {
            width: node_width.combine_explicit_with_child(combined_width),
            height: node_height.combine_explicit_with_child(combined_height),
            expand_x: node.constraints.expand_x || any_expand_x,
            expand_y: node.constraints.expand_y || any_expand_y,
        }
    }

    fn calculate_node_constraints(
        &self,
        node_id: NodeId,
        _state: &mut T,
        _ui_state: &mut U,
    ) -> SizeConstraints {
        let node = &self.nodes[node_id];

        let node_width = Constraint::new(node.constraints.width_min, node.constraints.width_max);
        let node_height = Constraint::new(node.constraints.height_min, node.constraints.height_max);

        match &node.node_type {
            NodeType::Draw(_) => SizeConstraints {
                width: node_width,
                height: node_height,
                expand_x: node.constraints.expand_x,
                expand_y: node.constraints.expand_y,
            },
            NodeType::Explicit => {
                if let Some(&child_id) = node.children.first() {
                    let child_hash = self.nodes[child_id].content_hash;
                    if let Some(&child_constraints) = self.cache.constraint_results.get(&child_hash)
                    {
                        SizeConstraints {
                            width: node_width.combine_explicit_with_child(child_constraints.width),
                            height: node_height
                                .combine_explicit_with_child(child_constraints.height),
                            expand_x: node.constraints.expand_x
                                || child_constraints.should_expand_x(),
                            expand_y: node.constraints.expand_y
                                || child_constraints.should_expand_y(),
                        }
                    } else {
                        SizeConstraints {
                            width: node_width,
                            height: node_height,
                            expand_x: node.constraints.expand_x,
                            expand_y: node.constraints.expand_y,
                        }
                    }
                } else {
                    SizeConstraints {
                        width: node_width,
                        height: node_height,
                        expand_x: node.constraints.expand_x,
                        expand_y: node.constraints.expand_y,
                    }
                }
            }
            NodeType::Column { spacing, .. } => self.combine_child_constraints(
                node,
                node_width,
                node_height,
                |combined_width, child_width, _| {
                    combined_width.combine_adjacent_priority(child_width)
                },
                |combined_height, child_height, spacing| {
                    combined_height.combine_sum(child_height, spacing)
                },
                *spacing,
            ),
            NodeType::Row { spacing, .. } => self.combine_child_constraints(
                node,
                node_width,
                node_height,
                |combined_width, child_width, spacing| {
                    combined_width.combine_sum(child_width, spacing)
                },
                |combined_height, child_height, _| {
                    combined_height.combine_adjacent_priority(child_height)
                },
                *spacing,
            ),
            NodeType::Stack { .. } => self.combine_child_constraints(
                node,
                node_width,
                node_height,
                |combined_width, child_width, _| {
                    combined_width.combine_adjacent_priority(child_width)
                },
                |combined_height, child_height, _| {
                    combined_height.combine_adjacent_priority(child_height)
                },
                0.0,
            ),
            NodeType::Padding(padding) => {
                if let Some(&child_id) = node.children.first() {
                    let child_hash = self.nodes[child_id].content_hash;
                    if let Some(&child_constraints) = self.cache.constraint_results.get(&child_hash)
                    {
                        let padding_width = padding.leading + padding.trailing;
                        let padding_height = padding.top + padding.bottom;

                        SizeConstraints {
                            width: Constraint::new(
                                child_constraints.width.lower.map(|l| l + padding_width),
                                child_constraints.width.upper.map(|u| u + padding_width),
                            ),
                            height: Constraint::new(
                                child_constraints.height.lower.map(|l| l + padding_height),
                                child_constraints.height.upper.map(|u| u + padding_height),
                            ),
                            expand_x: child_constraints.should_expand_x(),
                            expand_y: child_constraints.should_expand_y(),
                        }
                    } else {
                        SizeConstraints::default()
                    }
                } else {
                    SizeConstraints::default()
                }
            }
            _ => SizeConstraints {
                width: node_width,
                height: node_height,
                expand_x: node.constraints.expand_x,
                expand_y: node.constraints.expand_y,
            },
        }
    }

    fn allocate_node_area(&mut self, node_id: NodeId, state: &mut T, ui_state: &mut U) {
        let available_area = self.nodes[node_id].area;

        match &self.nodes[node_id].node_type {
            NodeType::Column {
                spacing,
                x_align: align,
                y_align: off_axis_align,
            } => {
                let spacing = *spacing;
                let align = *align;
                let off_axis_align = *off_axis_align;
                self.allocate_sequence_areas(
                    node_id,
                    available_area,
                    spacing,
                    true,
                    align,
                    off_axis_align,
                    state,
                    ui_state,
                );
                return;
            }
            NodeType::Row {
                spacing,
                y_align: align,
                x_align: off_axis_align,
            } => {
                let spacing = *spacing;
                let align = *align;
                let off_axis_align = *off_axis_align;
                self.allocate_sequence_areas(
                    node_id,
                    available_area,
                    spacing,
                    false,
                    off_axis_align,
                    align,
                    state,
                    ui_state,
                );
                return;
            }
            NodeType::Stack { x_align, y_align } => {
                let x_align = *x_align;
                let y_align = *y_align;
                self.allocate_stack_areas(
                    node_id,
                    available_area,
                    &x_align,
                    &y_align,
                    state,
                    ui_state,
                );
                return;
            }
            NodeType::Explicit => {
                if let Some(&child_id) = self.nodes[node_id].children.first() {
                    let constraints = self.nodes[node_id].constraints.clone();

                    let constrained_area = self.apply_constraints_to_area(
                        available_area,
                        &constraints,
                        XAlign::Center,
                        YAlign::Center,
                        state,
                        ui_state,
                    );

                    self.nodes[child_id].area = constrained_area;
                }
                return;
            }
            _ => {}
        }

        match &self.nodes[node_id].node_type {
            NodeType::Padding(padding) => {
                if let Some(&child_id) = self.nodes[node_id].children.first() {
                    let child_area = Area {
                        x: available_area.x + padding.leading,
                        y: available_area.y + padding.top,
                        width: (available_area.width - padding.leading - padding.trailing).max(0.0),
                        height: (available_area.height - padding.top - padding.bottom).max(0.0),
                    };
                    self.nodes[child_id].area = child_area;
                }
            }
            NodeType::Offset { x, y } => {
                if let Some(&child_id) = self.nodes[node_id].children.first() {
                    let child_area = Area {
                        x: available_area.x + x,
                        y: available_area.y + y,
                        width: available_area.width,
                        height: available_area.height,
                    };
                    self.nodes[child_id].area = child_area;
                }
            }
            NodeType::Visibility { .. } => {
                if let Some(&child_id) = self.nodes[node_id].children.first() {
                    self.nodes[child_id].area = available_area;
                }
            }
            NodeType::Coupled {
                over: _,
                element,
                coupled,
            } => {
                if self.nodes[node_id].children.len() >= 2 {
                    let element_id = self.nodes[node_id].children[*element];
                    let coupled_id = self.nodes[node_id].children[*coupled];

                    let element_hash = self.nodes[element_id].content_hash;
                    let mut element_constraints = if let Some(&size_constraints) =
                        self.cache.constraint_results.get(&element_hash)
                    {
                        let width = size_constraints.width.lower.unwrap_or(0.0);
                        let height = size_constraints.height.lower.unwrap_or(0.0);

                        NodeConstraints {
                            width_min: if width > 0.0 { Some(width) } else { None },
                            width_max: if width > 0.0 { Some(width) } else { None },
                            height_min: if height > 0.0 { Some(height) } else { None },
                            height_max: if height > 0.0 { Some(height) } else { None },
                            ..Default::default()
                        }
                    } else {
                        self.nodes[node_id].constraints.clone()
                    };

                    let element_node_constraints = &self.nodes[element_id].constraints;
                    if let Some(ref dynamic_width) = element_node_constraints.dynamic_width {
                        let calculated_width =
                            dynamic_width(available_area.height, state, ui_state).max(0.0);
                        element_constraints.width_min = Some(calculated_width);
                        element_constraints.width_max = Some(calculated_width);
                    }
                    if let Some(ref dynamic_height) = element_node_constraints.dynamic_height {
                        let calculated_height =
                            dynamic_height(available_area.width, state, ui_state).max(0.0);
                        element_constraints.height_min = Some(calculated_height);
                        element_constraints.height_max = Some(calculated_height);
                    }

                    let constrained_area = self.apply_constraints_to_area(
                        available_area,
                        &element_constraints,
                        XAlign::Center,
                        YAlign::Center,
                        state,
                        ui_state,
                    );

                    self.nodes[element_id].area = constrained_area;
                    self.nodes[coupled_id].area = constrained_area;
                }
            }

            _ => {
                let constraints = self.nodes[node_id].constraints.clone();
                let final_area = self.apply_constraints_to_area(
                    available_area,
                    &constraints,
                    XAlign::Center,
                    YAlign::Center,
                    state,
                    ui_state,
                );
                self.nodes[node_id].area = final_area;

                let children = self.nodes[node_id].children.clone();
                for &child_id in &children {
                    self.nodes[child_id].area = final_area;
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
        x_align: XAlign,
        y_align: YAlign,
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
            let constraints = &self.nodes[child_id].constraints;
            let mut lower = if is_vertical {
                constraints.height_min.or((constraints.dynamic_height)
                    .as_ref()
                    .map(|f| f(available_area.width, state, ui_state)))
            } else {
                constraints.width_min.or((constraints.dynamic_width)
                    .as_ref()
                    .map(|f| f(available_area.height, state, ui_state)))
            };
            let mut upper = if is_vertical {
                constraints.height_max.or((constraints.dynamic_height)
                    .as_ref()
                    .map(|f| f(available_area.width, state, ui_state)))
            } else {
                constraints.width_max.or((constraints.dynamic_width)
                    .as_ref()
                    .map(|f| f(available_area.height, state, ui_state)))
            };

            if lower.is_none() && upper.is_none() {
                let child_constraints = &self.nodes[child_id].constraints;
                let child_hash = self.nodes[child_id].content_hash;

                let is_expanded = if let Some(&size_constraints) =
                    self.cache.constraint_results.get(&child_hash)
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

                if !is_expanded {
                    if let Some(&size_constraints) = self.cache.constraint_results.get(&child_hash)
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
            }

            let mut final_size = None;

            if let Some(lower) = lower {
                if default_size < lower {
                    pool += default_size - lower;
                    final_size = Some(lower);
                }
            }
            if let Some(upper) = upper {
                if default_size > upper {
                    pool += default_size - upper;
                    final_size = Some(upper);
                }
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
            match y_align {
                YAlign::Top => available_area.y,
                YAlign::Center => available_area.y + (pool * 0.5),
                YAlign::Bottom => available_area.y + pool,
            }
        } else {
            match x_align {
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

            let base_area = if is_vertical {
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
            };

            let constraints = &self.nodes[child_id].constraints;

            let mut effective_constraints = constraints.clone();
            if let Some(&size_constraints) = self
                .cache
                .constraint_results
                .get(&self.nodes[child_id].content_hash)
            {
                match &self.nodes[child_id].node_type {
                    NodeType::Row { .. } => {
                        self.apply_intrinsic_constraints(
                            &mut effective_constraints,
                            &size_constraints,
                            constraints,
                            is_vertical,
                            true, // Row hugs cross-axis in column, main-axis in row
                            false,
                        );
                    }
                    NodeType::Column { .. } => {
                        self.apply_intrinsic_constraints(
                            &mut effective_constraints,
                            &size_constraints,
                            constraints,
                            is_vertical,
                            false, // Column hugs main-axis in column, cross-axis in row
                            true,
                        );
                    }
                    NodeType::Stack { .. } => {
                        self.apply_intrinsic_constraints(
                            &mut effective_constraints,
                            &size_constraints,
                            constraints,
                            is_vertical,
                            true, // Stack hugs cross-axis
                            false,
                        );
                    }
                    _ => {}
                }
            }

            let final_area = self.apply_constraints_to_area(
                base_area,
                &effective_constraints,
                x_align,
                y_align,
                state,
                ui_state,
            );

            self.nodes[child_id].area = final_area;

            current_pos += child_size + spacing;
        }
    }

    fn allocate_sequence_areas(
        &mut self,
        node_id: NodeId,
        available_area: Area,
        spacing: f32,
        is_vertical: bool,
        main_align: Option<XAlign>,
        cross_align: Option<YAlign>,
        state: &mut T,
        ui_state: &mut U,
    ) {
        let children = self.nodes[node_id].children.clone();
        let x_align = main_align.unwrap_or(XAlign::Center);
        let y_align = cross_align.unwrap_or(YAlign::Center);

        self.layout_axis(
            &children,
            spacing,
            available_area,
            is_vertical,
            x_align,
            y_align,
            state,
            ui_state,
        );
    }

    fn allocate_stack_areas(
        &mut self,
        node_id: NodeId,
        available_area: Area,
        x_align: &Option<XAlign>,
        y_align: &Option<YAlign>,
        state: &mut T,
        ui_state: &mut U,
    ) {
        let children = self.nodes[node_id].children.clone();
        let default_x_align = x_align.unwrap_or(XAlign::Center);
        let default_y_align = y_align.unwrap_or(YAlign::Center);

        let stack_constraints = self.nodes[node_id].constraints.clone();

        let stack_area = if stack_constraints.expand_x || stack_constraints.expand_y {
            let mut max_child_width = available_area.width;
            let mut max_child_height = available_area.height;

            for &child_id in &children {
                let child_constraints = &self.nodes[child_id].constraints;

                if let Some(child_width) =
                    child_constraints.width_min.or(child_constraints.width_max)
                {
                    max_child_width = max_child_width.max(child_width);
                }

                if let Some(child_height) = child_constraints
                    .height_min
                    .or(child_constraints.height_max)
                {
                    max_child_height = max_child_height.max(child_height);
                }
            }

            let constrained_stack = self.apply_constraints_to_area(
                Area {
                    x: available_area.x,
                    y: available_area.y,
                    width: max_child_width,
                    height: max_child_height,
                },
                &stack_constraints,
                default_x_align,
                default_y_align,
                state,
                ui_state,
            );

            self.apply_constraints_to_area(
                available_area,
                &NodeConstraints {
                    width_min: Some(constrained_stack.width),
                    width_max: Some(constrained_stack.width),
                    height_min: Some(constrained_stack.height),
                    height_max: Some(constrained_stack.height),
                    x_align: stack_constraints.x_align,
                    y_align: stack_constraints.y_align,
                    ..Default::default()
                },
                default_x_align,
                default_y_align,
                state,
                ui_state,
            )
        } else {
            self.apply_constraints_to_area(
                available_area,
                &stack_constraints,
                default_x_align,
                default_y_align,
                state,
                ui_state,
            )
        };

        for &child_id in &children {
            let constraints = self.nodes[child_id].constraints.clone();

            let has_constraints = constraints.width_min.is_some()
                || constraints.width_max.is_some()
                || constraints.height_min.is_some()
                || constraints.height_max.is_some();

            if has_constraints {
                let final_area = self.apply_constraints_to_area(
                    stack_area,
                    &constraints,
                    default_x_align,
                    default_y_align,
                    state,
                    ui_state,
                );
                self.nodes[child_id].area = final_area;
            } else {
                self.nodes[child_id].area = stack_area;
            }
        }
    }

    fn apply_constraints_to_area(
        &mut self,
        area: Area,
        constraints: &NodeConstraints<T, U>,
        contextual_x_align: XAlign,
        contextual_y_align: YAlign,
        state: &mut T,
        ui_state: &mut U,
    ) -> Area {
        let mut width = area.width;
        let mut height = area.height;

        if let Some(ref dynamic_width) = constraints.dynamic_width {
            width = dynamic_width(area.height, state, ui_state).max(0.0);
        }
        if let Some(ref dynamic_height) = constraints.dynamic_height {
            height = dynamic_height(area.width, state, ui_state).max(0.0);
        }

        if let Some(width_min) = constraints.width_min {
            if let Some(width_max) = constraints.width_max {
                width = width.clamp(width_min, width_max.max(width_min));
            } else {
                width = width.max(width_min);
            }
        } else if let Some(width_max) = constraints.width_max {
            width = width.min(width_max);
        }

        if let Some(height_min) = constraints.height_min {
            if let Some(height_max) = constraints.height_max {
                height = height.clamp(height_min, height_max.max(height_min));
            } else {
                height = height.max(height_min);
            }
        } else if let Some(height_max) = constraints.height_max {
            height = height.min(height_max);
        }

        let x_align = constraints.x_align.unwrap_or(contextual_x_align);
        let y_align = constraints.y_align.unwrap_or(contextual_y_align);

        let x = match x_align {
            XAlign::Leading => area.x,
            XAlign::Trailing => area.x + (area.width - width),
            XAlign::Center => area.x + (area.width * 0.5) - (width * 0.5),
        };

        let y = match y_align {
            YAlign::Top => area.y,
            YAlign::Bottom => area.y + (area.height - height),
            YAlign::Center => area.y + (area.height * 0.5) - (height * 0.5),
        };

        Area {
            x,
            y,
            width,
            height,
        }
    }

    fn draw_iterative(
        &mut self,
        root_id: NodeId,
        state: &mut T,
        ui_state: &mut U,
        contextual_visibility: bool,
    ) {
        let mut stack = vec![(root_id, contextual_visibility)];

        while let Some((node_id, visible)) = stack.pop() {
            let node_type = &self.nodes[node_id].node_type;
            let children = self.nodes[node_id].children.clone();
            let area = self.nodes[node_id].area;

            match node_type {
                NodeType::Draw(draw_fn) => {
                    if visible {
                        draw_fn(area, state, ui_state);
                    }
                }
                NodeType::Visibility {
                    visible: node_visible,
                } => {
                    let effective_visibility = visible && *node_visible;
                    for &child_id in children.iter() {
                        stack.push((child_id, effective_visibility));
                    }
                }
                NodeType::Explicit => {
                    for &child_id in children.iter().rev() {
                        stack.push((child_id, visible));
                    }
                }
                NodeType::Coupled {
                    over,
                    element,
                    coupled,
                } => {
                    let element_id = children[*element];
                    let coupled_id = children[*coupled];

                    if *over {
                        stack.push((coupled_id, visible));
                        stack.push((element_id, visible));
                    } else {
                        stack.push((element_id, visible));
                        stack.push((coupled_id, visible));
                    }
                }
                _ => {
                    for &child_id in children.iter().rev() {
                        stack.push((child_id, visible));
                    }
                }
            }
        }
    }

    fn calculate_content_hash(&self, node_id: NodeId) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        node_id.hash(&mut hasher);
        self.nodes[node_id].children.len().hash(&mut hasher);
        hasher.finish()
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

pub fn stack_aligned<T, U>(
    x_align: Option<XAlign>,
    y_align: Option<YAlign>,
    elements: Vec<Node<T, U>>,
) -> Node<T, U> {
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
        computed: None,
    })
}

pub fn area_reader<T, U>(
    func: impl Fn(Area, &mut T, &mut U) -> Node<T, U> + 'static,
) -> Node<T, U> {
    Node::new(NodeType::AreaReader(Box::new(func)))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        mvp_layout.debug_visualize(bounds, &mut (), &mut ());
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
    fn test_visibility() {
        let mut visible_count = 0;
        let layout_node = column(vec![
            draw(|_, count: &mut i32, _: &mut ()| {
                *count += 1;
            })
            .visible(true),
            draw(|_, count: &mut i32, _: &mut ()| {
                *count += 1;
            })
            .visible(false),
        ]);

        let mut mvp_layout = Layout::new(layout_node);
        mvp_layout.draw(
            Area::new(0.0, 0.0, 100.0, 100.0),
            &mut visible_count,
            &mut (),
        );
        assert_eq!(visible_count, 1);
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 10., 100.));
                        })
                        .width(10.)]),
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
                    row(vec![draw(|a, _, _| {
                        assert_eq!(a, Area::new(30., 0., 10., 100.));
                    })
                    .width(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(60., 0., 10., 100.));
                        })
                        .width(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 100., 10.));
                        })
                        .height(10.)]),
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
                    row(vec![draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 30., 100., 10.));
                    })
                    .height(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 60., 100., 10.));
                        })
                        .height(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 10., 50.));
                        })
                        .width(10.)]),
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
                    row(vec![draw(|a, _, _| {
                        assert_eq!(a, Area::new(45., 0., 10., 50.));
                    })
                    .width(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(90., 0., 10., 50.));
                        })
                        .width(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 0., 50., 10.));
                        })
                        .height(10.)]),
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
                    row(vec![draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., 45., 50., 10.));
                    })
                    .height(10.)]),
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
                        row(vec![draw(|a, _, _| {
                            assert_eq!(a, Area::new(0., 90., 50., 10.));
                        })
                        .height(10.)]),
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
                row(vec![draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                })
                .aspect_width(1.)])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![draw(|a, _, _| {
                    assert_eq!(a, Area::new(25., 0., 50., 100.));
                })
                .aspect_width(0.5)])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                column(vec![draw(|a, _, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 100.));
                })
                .aspect_width(0.5)
                .align(Align::Leading)])
                .expand()
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![draw(|a, _, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 100.));
                })
                .aspect_width(0.5)
                .align(Align::Trailing)])
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
                stack(vec![draw(|a, _, _| {
                    assert_eq!(a, Area::new(30., 10., 40., 80.));
                })
                .aspect_width(0.5)
                .pad(10.)])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
            Layout::new({
                stack(vec![draw(|a, _, _| {
                    assert_eq!(a, Area::new(35., 10., 30., 80.));
                })
                .pad(10.)
                .aspect_width(0.5)])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
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
                column(vec![stack(vec![
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., -50., 100., 200.));
                    })
                    .height(200.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(0., -50., 100., 200.));
                    }),
                ])
                .expand()])
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
                        assert_eq!(a, Area::new(20., 25., 30., 50.));
                    })
                    .width(30.)
                    .height(50.),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(50., 0., 25., 100.));
                    }),
                    draw(|a, _, _| {
                        assert_eq!(a, Area::new(75., 0., 25., 100.));
                    }),
                ])
            })
            .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
        }
    }
}
