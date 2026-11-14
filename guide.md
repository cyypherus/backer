# Laying out views with Backer

Children report size requirements; parents allocate space. Build layouts by deciding which content determines size, which containers accept extra room, and which decoration follows. Derive dimensions from those relationships instead of guessing them.

This guide targets Backer 0.12.0. Examples assume `use backer::{Align, nodes::*};` and prebuilt layout nodes.

## Expansion belongs to a specific node

Expanding children does not necessarily expand their parent. In the dashboard table, expanding columns left the enclosing row hugging its contents and centering them inside a wider border. The row itself needed `expand_x()`.

Follow the allocation from the root to the intended flexible region. Use `expand_x` or `expand_y` on the containers that should accept extra room on that axis. Expansion preserves minimum requirements; it does not make oversized content fit.

## A spacer affects both axes unless told otherwise

An unconstrained `draw` or `space` can make its container expand. A horizontal spacer should usually be excluded from vertical sizing:

```rust
row(vec![title, space().inert_y(), actions]).expand_x()
```

Use `space().inert_x()` for the corresponding vertical relationship. Fully `inert()` children reserve no share on the sequence axis, so a fully inert spacer cannot push siblings apart.

## Content determines size; decoration follows

```rust
stack(vec![
    background.inert(),
    content.pad(24.),
])
```

An unconstrained background otherwise participates in sizing and can make a compact card fill its parent. `inert` excludes its requirements; it does not disable drawing or remove its own constraints. Use a stack when views should occupy the same region.

## Sizing and placement are separate

Equal-height cards did not give their content matching top edges. Shorter content groups were centered inside the taller cards.

`column_aligned(Align::Top, ...)` aligns that column's children. To position the column inside its surrounding stack, align the column itself:

```rust
column_spaced(12., children).pad(16.).align(Align::Top)
```

Container alignment controls immediate children and takes precedence on its specified axes. Alignment has no visible effect when a node already fills its allocation.

## Measure requirements instead of copying dimensions

Backer does not measure text. Supply measured intrinsic sizes, or use `dynamic_height` to derive wrapped height from allocated width. `dynamic_width` receives height. Measurement callbacks can run repeatedly; keep their results deterministic.

Independent table columns also do not automatically share row heights. Measure every cell with `min_height`, take each row's maximum, and use that measured height across the row. Matching padding is insufficient when cells contain different typography, badges, or icons.

## Change the arrangement when content cannot fit

Rows and columns do not automatically reflow. Choose another tree at the owning layout boundary; the dashboard changes from four metric columns to two. Keep table overflow scrollable instead of compressing measured content or silently clipping it.

Use padding and spacing for insets and gaps. Use aspect methods for genuine dimension dependencies. Numeric bounds need a design requirement, not a screenshot coordinate. Check actual allocations and rendered output with different widths, heights, and content lengths before trusting a rule.
