use backer::nodes::*;
use backer::{Align, Area, Layout};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HighlightedCase {
    RelAbsSequence,
    AlignmentOffset,
    None,
}

#[derive(Clone, Copy, Debug)]
struct State {
    highlight: HighlightedCase,
}

enum Drawable {
    Rect {
        area: Area,
        color: Color,
    },
    Text {
        area: Area,
        text: &'static str,
        font_size: f32,
        color: Color,
    },
    Button {
        area: Area,
        label: &'static str,
        action: Box<dyn Fn(&mut State) + 'static>,
    },
}

const BTN_SIZE: f32 = 50.;

#[macroquad::main("Demo")]
async fn main() {
    let mut state = State {
        highlight: HighlightedCase::None,
    };

    loop {
        clear_background(BLACK);

        let available = Area {
            x: 0.,
            y: 0.,
            width: screen_width(),
            height: screen_height(),
        };

        let mut layout = layout_for_highlight(state.highlight);
        let commands = layout.draw(available);
        process_commands(commands, &mut state);

        next_frame().await;
    }
}

fn process_commands(commands: Vec<Drawable>, state: &mut State) {
    for command in commands {
        match command {
            Drawable::Rect { area, color } => {
                draw_rectangle(area.x, area.y, area.width, area.height, color);
            }
            Drawable::Text {
                area,
                text,
                font_size,
                color,
            } => {
                let dimensions = measure_text(text, None, font_size as u16, 1.0);
                let x = area.x + (area.width - dimensions.width) * 0.5;
                let y = area.y + (area.height + dimensions.height) * 0.5;
                draw_text(text, x, y, font_size, color);
            }
            Drawable::Button {
                area,
                label,
                action,
            } => {
                if widgets::Button::new(label)
                    .size(vec2(area.width, area.height))
                    .position(vec2(area.x, area.y))
                    .ui(&mut root_ui())
                {
                    (action)(state);
                }
            }
        }
    }
}

fn layout_for_highlight(highlight: HighlightedCase) -> Layout<Drawable> {
    row_spaced(
        10.,
        vec![
            match highlight {
                HighlightedCase::AlignmentOffset => empty(),
                HighlightedCase::RelAbsSequence | HighlightedCase::None => rel_abs_seq(),
            },
            match highlight {
                HighlightedCase::RelAbsSequence => empty(),
                HighlightedCase::AlignmentOffset | HighlightedCase::None => {
                    alignment_offset_section()
                }
            },
        ],
    )
}

fn rel_abs_seq() -> Layout<Drawable> {
    column_spaced(
        10.,
        vec![
            text("Mixed (rel/abs) Sequence Constraints", 15., WHITE),
            stack(vec![
                rect(BLUE),
                column_spaced(10., vec![rect(WHITE), rect(WHITE).height(30.), rect(WHITE)])
                    .pad(10.),
            ]),
            button("Fullscreen", |state: &mut State| {
                if state.highlight == HighlightedCase::RelAbsSequence {
                    state.highlight = HighlightedCase::None;
                } else {
                    state.highlight = HighlightedCase::RelAbsSequence;
                }
            })
            .height(BTN_SIZE)
            .align(Align::Bottom),
        ],
    )
}

fn alignment_offset_section() -> Layout<Drawable> {
    column_spaced(
        10.,
        vec![
            text("Alignment & Offset", 15., WHITE),
            stack(vec![
                rect(BLUE),
                rect(WHITE).height(30.).width(30.).align(Align::Leading),
                rect(WHITE).height(30.).width(30.).align(Align::Trailing),
                rect(WHITE).height(30.).width(30.).align(Align::Top),
                rect(WHITE).height(30.).width(30.).align(Align::Bottom),
                rect(WHITE).height(30.).width(30.).align(Align::TopLeading),
                rect(WHITE)
                    .height(30.)
                    .width(30.)
                    .align(Align::BottomLeading),
                rect(WHITE)
                    .height(30.)
                    .width(30.)
                    .align(Align::BottomTrailing),
                rect(WHITE).height(30.).width(30.).align(Align::TopTrailing),
                rect(WHITE)
                    .height(30.)
                    .width(30.)
                    .align(Align::CenterCenter)
                    .offset(10., 10.),
                rect(WHITE)
                    .height(30.)
                    .width(30.)
                    .align(Align::CenterCenter)
                    .offset(-10., -10.),
            ]),
            button("Fullscreen", |state: &mut State| {
                if state.highlight == HighlightedCase::AlignmentOffset {
                    state.highlight = HighlightedCase::None;
                } else {
                    state.highlight = HighlightedCase::AlignmentOffset;
                }
            })
            .height(BTN_SIZE)
            .align(Align::Bottom),
        ],
    )
}

fn text(string: &'static str, font_size: f32, color: Color) -> Layout<Drawable> {
    let dimensions = measure_text(string, None, font_size as u16, 1.0);
    draw(move |area: Area| Drawable::Text {
        area,
        text: string,
        font_size,
        color,
    })
    .width_range(200.0..)
    .height(dimensions.height)
}

fn rect(color: Color) -> Layout<Drawable> {
    draw(move |area: Area| Drawable::Rect { area, color })
}

fn button(label: &'static str, action: impl Fn(&mut State) + 'static) -> Layout<Drawable> {
    draw(move |area: Area| Drawable::Button {
        area,
        label,
        action: Box::new(action),
    })
}
