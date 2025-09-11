use crate::functional_layout::types::*;

pub fn resolve_constraints<A>(tree: InputTree<A>) -> ConstrainedTree<A> {
    let resolved_children: Vec<ConstrainedTree<A>> =
        tree.children.into_iter().map(resolve_constraints).collect();

    let resolved = calculate_constraints(&tree.layout, &tree.constraints, &resolved_children);

    ConstrainedTree {
        data: tree.data,
        layout: tree.layout,
        constraints: resolved,
        children: resolved_children,
    }
}

impl<A> InputTree<A> {
    pub fn cata<B, F>(self, f: &F) -> B
    where
        F: Fn(A, LayoutType, Constraints, Vec<B>) -> B,
    {
        let child_results: Vec<B> = self
            .children
            .into_iter()
            .map(|child| child.cata(f))
            .collect();

        f(self.data, self.layout, self.constraints, child_results)
    }
}

fn calculate_constraints<A>(
    layout: &LayoutType,
    input: &Constraints,
    children: &[ConstrainedTree<A>],
) -> Constraints {
    todo!()
}
