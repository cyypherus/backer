use backer::{models::*, nodes::*, Layout, Node};
use egui::{
    include_image, text::LayoutJob, Align as EguiAlign, Button, Color32, Image, ImageSource, Label,
    Pos2, Rect, Stroke, Ui,
};

#[derive(Default)]
pub struct TemplateApp {
    zoom_set: bool,
    web: bool,
    sidebar: bool,
}

impl TemplateApp {
    pub fn new(web: bool) -> Self {
        Self {
            zoom_set: false,
            web,
            sidebar: false,
        }
    }
}

struct State<'a> {
    ui: &'a mut Ui,
    sidebar: &'a mut bool,
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.web && !self.zoom_set && ctx.screen_rect().size().x < 600. {
            self.zoom_set = true;
            let base_width = 600.0;
            let current_width = ctx.screen_rect().size().x;
            let zoom_factor = current_width / base_width;
            ctx.set_zoom_factor(zoom_factor);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut layout = Layout::new(my_layout_fn());
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            let mut state = State {
                ui: &mut *ui,
                sidebar: &mut self.sidebar,
            };
            layout.draw(available_area, &mut state);
        });
    }
}

const DEMO_BG: Color32 = Color32::from_rgb(25, 25, 27);
const DEMO_GRAY: Color32 = Color32::from_rgb(50, 50, 50);
const DEMO_DESTRUCTIVE: Color32 = Color32::from_rgb(255, 100, 100);
const DEMO_DESTRUCTIVE_SECONDARY: Color32 = Color32::from_rgb(210, 40, 40);
const DEMO_HINT: Color32 = Color32::from_rgb(35, 35, 38);
const DEMO_FG: Color32 = Color32::from_rgb(250, 250, 255);
const DEMO_FG_SECONDARY: Color32 = Color32::from_rgb(180, 180, 183);

fn my_layout_fn<'a>() -> Node<'a, State<'a>> {
    dynamic(|state| {
        stack(vec![
            rect(Color32::TRANSPARENT, DEMO_BG, 0.),
            row(vec![
                row_divider(DEMO_GRAY).width(1.),
                column(vec![
                    header(state),
                    col_divider(DEMO_GRAY).height(1.),
                    main_view(state),
                    col_divider(DEMO_GRAY).height(1.),
                    footer(state),
                ]),
            ])
            .align(Align::Top),
            if *state.sidebar {
                side_bar(state)
            } else {
                empty()
            },
        ])
    })
}

fn footer<'a>(state: &mut State<'_>) -> Node<'a, State<'a>> {
    row_spaced(
        10.,
        vec![
            row_spaced(
                20.,
                vec![
                    label_color(state, "Game", 9., DEMO_FG_SECONDARY),
                    label_color(state, "Terms & Conditions", 9., DEMO_FG_SECONDARY),
                    label_color(state, "Privacy Policy", 9., DEMO_FG_SECONDARY),
                ],
            )
            .align(Align::Leading),
            space(),
            label_color(
                state,
                "© Backer 2021. All rights reserved",
                9.,
                DEMO_FG_SECONDARY,
            )
            .width_range((150.)..),
        ],
    )
    .pad(10.)
    .height(40.)
}

fn main_view<'a>(state: &mut State<'_>) -> Node<'a, State<'a>> {
    let profile_blurb = "Your public profile URL can be shared with anyone and allows them to immediately see your bases and activity in Backer.";
    let pic_blurb = "Upload a profile picture of yourself or the character you always wanted to be. Your avatar will be displayed all over the Backer world.";
    let info_blurb = "Tell the world about yourself. Information you add will be visible only in your profile, not for all users.";
    stack(vec![stack(vec![
        rect(DEMO_GRAY, DEMO_HINT, 5.),
        column_spaced(
            10.,
            vec![
                row_spaced(
                    10.,
                    vec![
                        column_spaced(
                            10.,
                            vec![
                                label(state, "Public profile", 18.),
                                fit_label(state, profile_blurb, 10.),
                            ],
                        )
                        .align_contents(Align::Leading)
                        .width_range((80.0)..),
                        column_spaced(
                            10.,
                            vec![
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    row_spaced(
                                        10.,
                                        vec![
                                            label_color(
                                                state,
                                                "ejjonny.io/backer/username",
                                                12.,
                                                DEMO_FG_SECONDARY,
                                            ),
                                            icon(include_image!("../assets/copy.svg")).aspect(1.),
                                        ],
                                    )
                                    .pad(3.),
                                ])
                                .height(25.),
                                row_spaced(
                                    10.,
                                    vec![
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            row_spaced(
                                                10.,
                                                vec![
                                                    icon(include_image!("../assets/share.svg"))
                                                        .aspect(1.),
                                                    label_color(
                                                        state,
                                                        "Share",
                                                        12.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            )
                                            .pad(3.),
                                        ])
                                        .height(25.),
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            row_spaced(
                                                10.,
                                                vec![
                                                    icon(include_image!("../assets/map-pin.svg"))
                                                        .aspect(1.),
                                                    label_color(
                                                        state,
                                                        "View location",
                                                        12.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            )
                                            .pad(3.),
                                        ])
                                        .height(25.),
                                    ],
                                ),
                            ],
                        ),
                    ],
                )
                .pad_bottom(15.),
                col_divider(DEMO_GRAY).height(1.),
                row_spaced(
                    10.,
                    vec![
                        column_spaced(
                            10.,
                            vec![
                                label(state, "Edit PFP", 18.),
                                fit_label(state, pic_blurb, 10.),
                            ],
                        )
                        .align_contents(Align::Leading)
                        .width_range((80.0)..),
                        column_spaced(
                            10.,
                            vec![
                                row_spaced(
                                    10.,
                                    vec![
                                        rect(DEMO_FG, DEMO_BG, 100.).height(30.).width(30.),
                                        column_spaced(
                                            5.,
                                            vec![
                                                label(state, "@UserName", 12.),
                                                label_color(
                                                    state,
                                                    "Living, laughing, loving",
                                                    10.,
                                                    DEMO_FG_SECONDARY,
                                                ),
                                            ],
                                        )
                                        .align_contents(Align::Leading),
                                    ],
                                ),
                                row_spaced(
                                    10.,
                                    vec![
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            label_color(state, "Upload", 12., DEMO_FG_SECONDARY)
                                                .pad(5.),
                                        ])
                                        .height(25.),
                                        stack(vec![
                                            rect(DEMO_DESTRUCTIVE_SECONDARY, DEMO_BG, 5.),
                                            label_color(state, "Remove", 12., DEMO_DESTRUCTIVE)
                                                .pad(5.),
                                        ])
                                        .height(25.),
                                    ],
                                ),
                            ],
                        ),
                    ],
                )
                .pad_bottom(15.),
                col_divider(DEMO_GRAY).height(1.),
                row_spaced(
                    10.,
                    vec![
                        column_spaced(
                            10.,
                            vec![
                                label(state, "Edit personal information", 18.),
                                fit_label(state, info_blurb, 10.),
                            ],
                        )
                        .align_contents(Align::Leading)
                        .width_range((80.0)..),
                        column_spaced(
                            5.,
                            vec![
                                label_color(state, "Edit username", 12., DEMO_FG_SECONDARY),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    fit_label_color(state, "@UserName", 12., DEMO_FG)
                                        .align(Align::Leading)
                                        .pad(5.),
                                ])
                                .height(25.),
                                label_color(state, "Bio", 12., DEMO_FG_SECONDARY),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    label_color(state, "Living, laughing, loving", 12., DEMO_FG)
                                        .align(Align::TopLeading)
                                        .pad(5.),
                                ])
                                .align_contents(Align::TopLeading)
                                .height(50.),
                            ],
                        )
                        .align_contents(Align::Leading),
                    ],
                ),
            ],
        )
        .align_contents(Align::TopLeading)
        .expand()
        .pad_y(40.)
        .pad_x(30.),
        rect_stroke(DEMO_GRAY),
    ])
    .pad(20.)])
}

fn side_bar<'a>(state: &mut State<'_>) -> Node<'a, State<'a>> {
    stack(vec![
        rect(Color32::TRANSPARENT, DEMO_BG, 0.),
        column_spaced(
            15.,
            vec![
                row_spaced(
                    10.,
                    vec![menu_button(state), label(state, "BACKER", 22.).height(35.)],
                ),
                col_divider(DEMO_GRAY).pad_x(-30.).height(1.),
                label(state, "Home", 10.),
                label(state, "Explore", 10.),
                label(state, "Marketplace", 10.),
                label(state, "My Account", 10.),
                col_divider(DEMO_GRAY).pad_trailing(-20.).height(1.),
                label(state, "Activity", 10.),
                label(state, "News", 10.),
                label(state, "Docs", 10.),
                col_divider(DEMO_GRAY).pad_trailing(-20.).height(1.),
                label(state, "Twitter", 10.),
                label(state, "Telegram", 10.),
                label(state, "Medium", 10.),
                space(),
            ],
        )
        .align_contents(Align::TopLeading)
        .pad(30.),
    ])
    .align(Align::Leading)
    .width(200.)
}

fn header<'a>(state: &mut State<'_>) -> Node<'a, State<'a>> {
    row_spaced(
        10.,
        vec![
            menu_button(state),
            label(state, "My Account", 18.).width(110.),
            space(),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                label(state, "$115,000", 12.),
            ])
            .width(80.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                row(vec![label(state, "Operational", 12.)]),
            ])
            .width(90.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/bell.svg")).pad_y(8.5),
            ])
            .aspect(1.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/user.svg")).pad_y(8.5),
            ])
            .aspect(1.),
        ],
    )
    .pad_top(35.)
    .pad_bottom(15.)
    .pad_x(30.)
    .height(80.)
}

fn menu_button<'a>(_ui: &mut State<'_>) -> Node<'a, State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        if ui
            .ui
            .put(
                rect_from(area),
                Button::image(include_image!("../assets/menu-scale.svg"))
                    .fill(Color32::TRANSPARENT),
            )
            .clicked()
        {
            *ui.sidebar = !*ui.sidebar;
        }
    })
    .aspect(1.)
    .width(30.)
    .height(30.)
}

fn icon<'a>(image: impl Into<ImageSource<'static>> + 'static) -> Node<'a, State<'a>> {
    let image = Image::new(image).tint(Color32::WHITE);
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.put(rect_from(area), image.clone());
    })
}

fn label<'a, S: AsRef<str> + 'static + Clone + Copy>(
    state: &mut State<'_>,
    text: S,
    size: f32,
) -> Node<'a, State<'a>> {
    label_common(state, text, size, false, Color32::WHITE)
}

fn fit_label<'a, S: AsRef<str> + 'static + Clone + Copy>(
    state: &mut State<'_>,
    text: S,
    size: f32,
) -> Node<'a, State<'a>> {
    label_common(state, text, size, true, Color32::WHITE)
}

fn fit_label_color<'a, S: AsRef<str> + 'static + Clone + Copy>(
    state: &mut State<'_>,
    text: S,
    size: f32,
    color: Color32,
) -> Node<'a, State<'a>> {
    label_common(state, text, size, true, color)
}

fn label_color<'a, S: AsRef<str> + 'static + Clone + Copy>(
    state: &mut State<'_>,
    text: S,
    size: f32,
    color: Color32,
) -> Node<'a, State<'a>> {
    label_common(state, text, size, false, color)
}

fn label_common<'a, S: AsRef<str> + 'static + Clone + Copy>(
    state: &mut State<'_>,
    text: S,
    size: f32,
    fit_width: bool,
    color: Color32,
) -> Node<'a, State<'a>> {
    fn layout_job(
        font_size: f32,
        width: f32,
        text: String,
        align: EguiAlign,
        color: Color32,
    ) -> LayoutJob {
        let mut job = LayoutJob::single_section(
            text.clone(),
            egui::TextFormat {
                font_id: egui::FontId::new(font_size, egui::FontFamily::Proportional),
                extra_letter_spacing: 0.,
                line_height: Some(14.),
                color,
                background: Color32::TRANSPARENT,
                italics: false,
                underline: Stroke::NONE,
                strikethrough: Stroke::NONE,
                valign: align,
            },
        );
        job.wrap.max_width = width;
        job
    }
    let text = text.as_ref().to_string();
    let text_dyn = text.clone();
    if fit_width {
        draw(move |area, ui: &mut State<'_>| {
            let job = layout_job(size, area.width, text.clone(), EguiAlign::Min, color);
            let rect = rect_from(area);
            ui.ui.allocate_ui_at_rect(rect, |ui| {
                ui.vertical(|ui| {
                    ui.add(Label::new(job.clone()));
                })
            });
        })
        .dynamic_height(move |width, state| {
            let galley_text = text_dyn.clone();
            let galley_size = state
                .ui
                .fonts(move |fonts| {
                    fonts.layout_job(layout_job(size, width, galley_text, EguiAlign::Min, color))
                })
                .size();
            galley_size.y
        })
    } else {
        let galley_text = text.clone();
        let galley_size = state
            .ui
            .fonts(move |fonts| {
                fonts.layout_job(layout_job(
                    size,
                    300.,
                    galley_text,
                    EguiAlign::Center,
                    color,
                ))
            })
            .size();
        draw(move |area, ui: &mut State<'_>| {
            let job = layout_job(size, 300., text.clone(), EguiAlign::Center, color);
            ui.ui.put(rect_from(area), Label::new(job.clone()));
        })
        .height(galley_size.y)
        .width(galley_size.x)
    }
}

fn col_divider<'a>(color: Color32) -> Node<'a, State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.painter().line_segment(
            [
                Pos2::new(area.x, area.y + (area.height * 0.5)),
                Pos2::new(area.x + area.width, area.y + (area.height * 0.5)),
            ],
            Stroke::new(1., color),
        );
    })
}

fn row_divider<'a>(color: Color32) -> Node<'a, State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.painter().line_segment(
            [
                Pos2::new(area.x + (area.width * 0.5), area.y),
                Pos2::new(area.x + (area.width * 0.5), area.y + area.height),
            ],
            Stroke::new(1., color),
        );
    })
}

fn rect<'a>(stroke: Color32, fill: Color32, rounding: f32) -> Node<'a, State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui
            .painter()
            .rect_stroke(rect_from(area), rounding, Stroke::new(1., stroke));
        ui.ui.painter().rect_filled(rect_from(area), rounding, fill);
    })
}

fn rect_stroke<'a>(color: Color32) -> Node<'a, State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui
            .painter()
            .rect_stroke(rect_from(area), 5., Stroke::new(1., color));
    })
}

fn area_from(rect: Rect) -> Area {
    Area {
        x: rect.min.x,
        y: rect.min.y,
        width: rect.max.x - rect.min.x,
        height: rect.max.y - rect.min.y,
    }
}

fn rect_from(area: Area) -> Rect {
    Rect {
        min: Pos2::new(area.x, area.y),
        max: Pos2::new(area.x + area.width, area.y + area.height),
    }
}
