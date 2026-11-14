# Layout guide

A child reports the size it needs. Its parent gives it space.

Let content set the size. Use fixed widths and heights only when the design requires them.

This guide uses Backer 0.12.0. Examples assume existing nodes and `use backer::{Align, nodes::*};`.

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

Equal card heights do not give their contents the same position.

Use `align()` to place a column inside its parent:

```rust
column(children).pad(16.).align(Align::Top)
```

Use `column_aligned()` to place the children inside that column.

## Measure content

Give Backer the text size measured by your text system.

Use `dynamic_height()` when height depends on width. Use `dynamic_width()` when width depends on height.

For a table, measure each cell with `min_height()`. Give all cells in a row the largest measured height.

## Handle small spaces

Backer keeps minimum sizes. If content does not fit, wrap text, add scrolling, or change the layout.

Rows and columns do not change their arrangement automatically.

Check small and large windows. Check longer text.
