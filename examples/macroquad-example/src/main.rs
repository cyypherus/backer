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
    let mut layout = Layout::new(layout_for_highlight());
    loop {
        layout.draw(
            Area::new(0., 0., screen_width(), screen_height()),
            &mut state,
            &mut (),
        );
        next_frame().await
    }
}

const BTN_SIZE: f32 = 50.;
fn layout_for_highlight<'n>() -> Node<'n, State, ()> {
    dynamic(|ctx: &mut State, _| {
        let highlight = ctx.highlight;
        row_spaced(
            10.,
            vec![
                if highlight == HighlightedCase::RelAbsSequence
                    || highlight == HighlightedCase::None
                {
                    scope(
                        |state: &mut State| &mut state.highlight,
                        |x| x,
                        rel_abs_seq(highlight),
                    )
                } else {
                    empty()
                },
                if highlight == HighlightedCase::AlignmentOffset
                    || highlight == HighlightedCase::None
                {
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
    })
}

fn rel_abs_seq<'n>(_highlight: HighlightedCase) -> Node<'n, HighlightedCase, ()> {
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

fn text<'n, T>(string: &'static str, font_size: f32, color: Color) -> Node<'n, T, ()> {
    let dimensions = measure_text(string, None, font_size as u16, 1.0);
    draw(move |area: Area, _: &mut T, _| {
        draw_text(
            string,
            area.absolute_pos.x + ((area.width - dimensions.width) * 0.5),
            area.absolute_pos.y + (area.height * 0.5) + (dimensions.height * 0.5),
            font_size,
            color,
        );
    })
    .width_range(200.0..)
    .height(dimensions.height)
}

fn rect<'n, T, U>(color: Color) -> Node<'n, T, U> {
    draw(move |area: Area, _: &mut T, _| {
        draw_rectangle(
            area.absolute_pos.x,
            area.absolute_pos.y,
            area.width,
            area.height,
            color,
        );
    })
}

fn button<'n, T, U, Action>(label: &'static str, action: Action) -> Node<'n, T, U>
where
    Action: Fn(&mut T) + 'static,
{
    draw(move |area: Area, ctx: &mut T, _| {
        if widgets::Button::new(label)
            .size(vec2(area.width, area.height))
            .position(vec2(area.absolute_pos.x, area.absolute_pos.y))
            .ui(&mut root_ui())
        {
            action(ctx);
        }
    })
}
