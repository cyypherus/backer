use backer::models::*;
use backer::nodes::*;
use backer::Layout;
use backer::Node;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;

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

#[macroquad::main("Demo")]
async fn main() {
    let mut state = State {
        highlight: HighlightedCase::None,
    };
    let layout = Layout::new(layout_for_highlight);
    loop {
        layout.draw(
            Area {
                x: 0.,
                y: 0.,
                width: screen_width(),
                height: screen_height(),
            },
            &mut state,
        );
        next_frame().await
    }
}

const BTN_SIZE: f32 = 50.;
fn layout_for_highlight(ctx: &mut State) -> Node<State> {
    let highlight = ctx.highlight;
    row_spaced(
        10.,
        vec![
            draw(|area, state: &mut State| {
                Layout::new(|state: &mut HighlightedCase| rel_abs_seq(*state))
                    .draw(area, &mut state.highlight);
            })
            .visible(
                highlight == HighlightedCase::RelAbsSequence || highlight == HighlightedCase::None,
            ),
            if highlight == HighlightedCase::AlignmentOffset || highlight == HighlightedCase::None {
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
                        button("Fullscreen", |ctx: &mut State| {
                            if ctx.highlight == HighlightedCase::AlignmentOffset {
                                ctx.highlight = HighlightedCase::None;
                            } else {
                                ctx.highlight = HighlightedCase::AlignmentOffset;
                            }
                        })
                        .height(BTN_SIZE)
                        .align(Align::Bottom),
                    ],
                )
            } else {
                empty()
            },
        ],
    )
}

fn rel_abs_seq(_highlight: HighlightedCase) -> Node<HighlightedCase> {
    column_spaced(
        10.,
        vec![
            text("Mixed (rel/abs) Sequence Constraints", 15., WHITE),
            stack(vec![
                rect(BLUE),
                column_spaced(10., vec![rect(WHITE), rect(WHITE).height(30.), rect(WHITE)])
                    .pad(10.),
            ]),
            button("Fullscreen", |highlight: &mut HighlightedCase| {
                if *highlight == HighlightedCase::RelAbsSequence {
                    *highlight = HighlightedCase::None;
                } else {
                    *highlight = HighlightedCase::RelAbsSequence;
                }
            })
            .height(BTN_SIZE)
            .align(Align::Bottom),
        ],
    )
}

fn text<U>(string: &'static str, font_size: f32, color: Color) -> Node<U> {
    let dimensions = measure_text(string, None, font_size as u16, 1.0);
    draw(move |area: Area, _: &mut U| {
        draw_text(
            string,
            area.x + ((area.width - dimensions.width) * 0.5),
            area.y + (area.height * 0.5) + (dimensions.height * 0.5),
            font_size,
            color,
        );
    })
    .width_range(200.0..)
    .height(dimensions.height)
}

fn rect<U>(color: Color) -> Node<U> {
    draw(move |area: Area, _: &mut U| {
        draw_rectangle(area.x, area.y, area.width, area.height, color);
    })
}

fn button<U, Action>(label: &'static str, action: Action) -> Node<U>
where
    Action: Fn(&mut U) + 'static,
{
    draw(move |area: Area, ctx: &mut U| {
        if widgets::Button::new(label)
            .size(vec2(area.width, area.height))
            .position(vec2(area.x, area.y))
            .ui(&mut root_ui())
        {
            action(ctx);
        }
    })
}
