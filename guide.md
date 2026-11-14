# Layout guide

A child reports the size it needs. Its parent gives it space.

Prefer inherent sizes over explicit widths and heights. Inherent sizes come from content.

Use expansion, flexible gaps, and alignment to fit the available space. Add an explicit size only when the design requires that size.

Examples assume existing nodes and `use backer::{Align, nodes::*};`.

## Use extra space

Use `expand_x()` for extra width. Use `expand_y()` for extra height.

Set expansion on each container that must grow. Do not assume that expanding a child also expands its parent.

## Keep gaps flexible

To push items apart in a row:

```rust
row(vec![title, space().inert_y(), actions]).expand_x()
```

The gap uses extra width without setting the row's height.

For a vertical gap, use `space().inert_x()` in a column.

## Let backgrounds follow content

```rust
stack(vec![background.inert(), content.pad(24.)])
```

The content sets the size. The background receives that space without setting the stack's size.

## Set position separately

To place content at the top of a card, align the content itself:

```rust
column(children).pad(16.).align(Align::Top)
```

Use `column_aligned()` to place the children inside that column.

## Check modifier order

Padding and offsets add a parent node. Later modifiers change that parent.

- `view.width(100.).pad(10.)`: set the content width, then add padding.
- `view.pad(10.).width(100.)`: set the width of the padded box.

Apply `inert()` after padding to exclude the whole padded view from its parent's size.

Repeated `width()` calls use the last value. Independent settings, such as width and height, can change order.

## Choose where overflow goes

When a view is larger than its space, alignment controls where it extends:

- `Leading`: past the right edge.
- `Trailing`: past the left edge.
- `CenterX`: past both edges.

`Top`, `Bottom`, and `CenterY` apply the same rule vertically.

Use `TopLeading` when overflow should extend to the right and bottom.

## Measure content

Give Backer the text size measured by your text system.

Use `dynamic_height()` when height depends on width. Use `dynamic_width()` when width depends on height.

For a table, measure each cell with `min_height()`. Give all cells in a row the largest measured height.

## Handle small spaces

If content does not fit, wrap the text, add scrolling, or change the layout.

Rows and columns do not change their arrangement automatically.

Check small and large windows. Check longer text.
