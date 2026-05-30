<div align="center">

# Backer

![rust](https://github.com/cyypherus/backer/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/backer.svg)](https://crates.io/crates/backer)
[![downloads](https://img.shields.io/crates/d/backer.svg)](https://crates.io/crates/backer)
[![license](https://img.shields.io/crates/l/backer.svg)](https://github.com/cyypherus/backer/blob/main/LICENSE)

**A library for straight-forward UI layout.**

</div>

Dependency free & framework-agnostic. Backer can be used in an index-based layout approach or with inline drawing code.

_This library **only** implements layout & would be most useful along with a GUI library that can do GUI things (like [macroquad](https://github.com/not-fl3/macroquad) or [egui](https://github.com/emilk/egui))._

### Features ✨

- Declarative API: The code should look like the structure it defines
- Minimal interface: No confusing overloads or magic, cascading effects
- Intuitive constraints: Backer adheres to & smoothly resolves size constraints with an advanced algorithm
- Performant: Layout can be comfortably recalculated every frame
- Easy integration & compatibility: Backer should work with just about any UI library with a bit of glue - so it isn't really fine-tuned for any specific UI solution.

This project intends to be a flexible layout tool & not much else.

# Quick Start

## 1. Define your views.

```rust
use backer::nodes::*;
use backer::{Area, Layout};

struct MyState;

enum View {
    Label { area: Area, text: &'static str },
}
```

## 2. Implement a `draw` node

The `View` in `Layout<'a, View, MyState>` is returned by `draw` nodes as `Vec<View>` and collected by `layout.draw`.

```rust
fn label<'a>(text: &'static str) -> Layout<'a, View, MyState> {
    draw(move |area: Area, _: &mut MyState| {
        // The `area` parameter is the space alotted for your view after layout is calculated
        // The `state` parameter is *your* global mutable state that you pass when you call layout (necessary things like text caches)
        // These returned values will be collected by `layout.draw`
        vec![View::Label { area, text }]
    })
}
```

## 3. Combine nodes to define & customize your layout

```rust
fn my_layout<'a>() -> Layout<'a, View, MyState> {
    row(vec![
        label("Hello"),
    ])
}

let mut layout = my_layout();
```

## 4. Draw your layout

```rust
// UI libraries generally will expose methods to get the available space for layout.
// In a real implementation this should use the real screen size!
let available_area = Area::new(
    0.,
    0.,
    400.,
    600.,
);
let mut my_state = MyState;

// Perform layout & collect the values returned by your draw nodes.
let views: Vec<View> = layout.draw(available_area, &mut my_state);
```

## Preview

Check out the [demo site](https://cyypherus.github.io/backer/): a mock page showcasing layout capabilities in a realistic interface. Built with [egui](https://github.com/emilk/egui)!

[<img src="https://github.com/user-attachments/assets/71c2e83c-67e0-46e9-9bb8-d3bc5926c973">](https://cyypherus.github.io/backer/)

Backer relies on simple rules that can compose to create complex, flexible layouts.

![stretched](https://github.com/user-attachments/assets/81fd3e70-a504-49c7-92b6-f4c6b05a5371)

<details>
<summary>See some code</summary>

```rust
    column_spaced(
        10.,
        vec![
            draw_a(ui),
            row_spaced(
                10.,
                vec![
                    draw_b(ui).width(180.).align(Align::Leading),
                    column_spaced(10., vec![draw_a(ui), draw_b(ui), draw_c(ui)]),
                ],
            ),
            draw_c(ui),
        ],
    )
    .pad(10.)
```

</details>

## Status

The crate is currently usable but new! Breaking changes may be relatively frequent as the crate matures.

## Contributing

> [!NOTE]
> This repo uses `cargo insta` to snapshot test the public API.
> If your PR changes the public API, one of the checks will fail by default.
> If the changes to the public API were intentional you can update the snapshot by running:
>
> `INSTA_UPDATE=always cargo test --features test-api`

Contributions are always welcome 🤗
