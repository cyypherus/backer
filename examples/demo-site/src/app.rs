#![allow(clippy::type_complexity)]

use backer::{Align, Area, Layout, nodes::*};
use egui::{
    Align as EguiAlign, Button, Color32, Image, ImageSource, Label, Pos2, Rect, Stroke, Ui,
    include_image, text::LayoutJob,
};
use std::sync::Arc;

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

enum Drawable {
    Action {
        area: Area,
        handler: Box<dyn Fn(&mut Ui, &mut bool, Area) + 'static>,
    },
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
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            let mut layout = build_layout(ctx, self.sidebar);
            let commands = layout.draw(available_area, &mut ());
            process_commands(commands, ui, &mut self.sidebar);
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

fn build_layout(ctx: &egui::Context, sidebar: bool) -> Layout<'static, Drawable, ()> {
    stack({
        let mut layers = vec![
            rect(Color32::TRANSPARENT, DEMO_BG, 0.),
            row(vec![
                row_divider(DEMO_GRAY).width(1.),
                column(vec![
                    header(ctx),
                    col_divider(DEMO_GRAY).height(1.),
                    main_view(ctx),
                    col_divider(DEMO_GRAY).height(1.),
                    footer(ctx),
                ]),
            ])
            .align(Align::Top),
        ];
        if sidebar {
            layers.push(side_bar(ctx));
        }
        layers
    })
}

fn footer(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    row_spaced(
        10.,
        vec![
            row_spaced(
                20.,
                vec![
                    label_color(ctx, "Game", 9., DEMO_FG_SECONDARY),
                    label_color(ctx, "Terms & Conditions", 9., DEMO_FG_SECONDARY),
                    label_color(ctx, "Privacy Policy", 9., DEMO_FG_SECONDARY),
                ],
            )
            .align(Align::Leading),
            space(),
            label_color(
                ctx,
                "© Backer 2021. All rights reserved",
                9.,
                DEMO_FG_SECONDARY,
            )
            .width_range(150.0..),
        ],
    )
    .pad(10.)
    .height(40.)
}

fn main_view(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    let profile_blurb = "Your public profile URL can be shared with anyone and allows them to immediately see your bases and activity in Backer.";
    let pic_blurb = "Upload a profile picture of yourself or the character you always wanted to be. Your avatar will be displayed all over the Backer world.";
    let info_blurb = "Tell the world about yourself. Information you add will be visible only in your profile, not for all users.";
    stack(vec![
        stack(vec![
            rect(DEMO_GRAY, DEMO_HINT, 5.),
            column_spaced(
                10.,
                vec![
                    row_spaced(
                        10.,
                        vec![
                            column_spaced_aligned(
                                10.,
                                Align::Leading,
                                vec![
                                    label(ctx, "Public profile", 18.),
                                    fit_label(ctx, profile_blurb, 10.),
                                ],
                            )
                            .width_range(80.0..),
                            column_spaced(
                                10.,
                                vec![
                                    stack(vec![
                                        rect(DEMO_FG, DEMO_BG, 5.),
                                        row_spaced(
                                            10.,
                                            vec![
                                                label_color(
                                                    ctx,
                                                    "cyypherus.io/backer/username",
                                                    12.,
                                                    DEMO_FG_SECONDARY,
                                                ),
                                                icon(include_image!("../assets/copy.svg"))
                                                    .aspect_width(1.)
                                                    .width(20.),
                                            ],
                                        )
                                        .pad(5.),
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
                                                            .aspect_width(1.)
                                                            .width(20.),
                                                        label_color(
                                                            ctx,
                                                            "Share",
                                                            12.,
                                                            DEMO_FG_SECONDARY,
                                                        )
                                                        .pad_trailing(5.),
                                                    ],
                                                )
                                                .pad(5.),
                                            ])
                                            .height(25.),
                                            stack(vec![
                                                rect(DEMO_FG, DEMO_BG, 5.),
                                                row_spaced(
                                                    10.,
                                                    vec![
                                                        icon(include_image!(
                                                            "../assets/map-pin.svg"
                                                        ))
                                                        .aspect_width(1.)
                                                        .width(20.),
                                                        label_color(
                                                            ctx,
                                                            "View location",
                                                            12.,
                                                            DEMO_FG_SECONDARY,
                                                        )
                                                        .pad_trailing(5.),
                                                    ],
                                                )
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
                            column_spaced_aligned(
                                10.,
                                Align::Leading,
                                vec![label(ctx, "Edit PFP", 18.), fit_label(ctx, pic_blurb, 10.)],
                            )
                            .width_range(80.0..),
                            column_spaced(
                                10.,
                                vec![
                                    row_spaced(
                                        10.,
                                        vec![
                                            rect(DEMO_FG, DEMO_BG, 100.).height(30.).width(30.),
                                            column_spaced_aligned(
                                                5.,
                                                Align::Leading,
                                                vec![
                                                    label(ctx, "@UserName", 12.),
                                                    label_color(
                                                        ctx,
                                                        "Living, laughing, loving",
                                                        10.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    row_spaced(
                                        10.,
                                        vec![
                                            stack(vec![
                                                rect(DEMO_FG, DEMO_BG, 5.),
                                                label_color(ctx, "Upload", 12., DEMO_FG_SECONDARY)
                                                    .pad(5.),
                                            ])
                                            .height(25.),
                                            stack(vec![
                                                rect(DEMO_DESTRUCTIVE_SECONDARY, DEMO_BG, 5.),
                                                label_color(ctx, "Remove", 12., DEMO_DESTRUCTIVE)
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
                            column_spaced_aligned(
                                10.,
                                Align::Leading,
                                vec![
                                    label(ctx, "Edit personal information", 18.),
                                    fit_label(ctx, info_blurb, 10.),
                                ],
                            )
                            .width_range(85.0..),
                            column_spaced(
                                5.,
                                vec![
                                    label_color(ctx, "Edit username", 12., DEMO_FG_SECONDARY),
                                    stack(vec![
                                        rect(DEMO_FG, DEMO_BG, 5.),
                                        fit_label_color(ctx, "@UserName", 12., DEMO_FG)
                                            .align(Align::Leading)
                                            .pad(5.),
                                    ])
                                    .height(25.),
                                    label_color(ctx, "Bio", 12., DEMO_FG_SECONDARY),
                                    stack(vec![
                                        rect(DEMO_FG, DEMO_BG, 5.),
                                        label_color(ctx, "Living, laughing, loving", 12., DEMO_FG)
                                            .align(Align::TopLeading)
                                            .pad(5.),
                                    ])
                                    .align(Align::TopLeading)
                                    .height(50.),
                                ],
                            )
                            .align(Align::Leading),
                        ],
                    ),
                ],
            )
            .align(Align::TopLeading)
            .expand()
            .pad_y(40.)
            .pad_x(30.),
            rect_stroke(DEMO_GRAY),
        ])
        .pad(20.),
    ])
}

fn side_bar(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    stack(vec![
        rect(Color32::TRANSPARENT, DEMO_BG, 0.),
        column_spaced(
            15.,
            vec![
                row_spaced(
                    10.,
                    vec![menu_button(), label(ctx, "BACKER", 22.).height(35.)],
                ),
                col_divider(DEMO_GRAY).pad_x(-30.).height(1.),
                label(ctx, "Home", 10.),
                label(ctx, "Explore", 10.),
                label(ctx, "Marketplace", 10.),
                label(ctx, "My Account", 10.),
                col_divider(DEMO_GRAY).pad_trailing(-20.).height(1.),
                label(ctx, "Activity", 10.),
                label(ctx, "News", 10.),
                label(ctx, "Docs", 10.),
                col_divider(DEMO_GRAY).pad_trailing(-20.).height(1.),
                label(ctx, "Twitter", 10.),
                label(ctx, "Telegram", 10.),
                label(ctx, "Medium", 10.),
                space(),
            ],
        )
        .align(Align::TopLeading)
        .pad(30.),
    ])
    .align(Align::Leading)
    .width(200.)
}

fn header(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    row_spaced(
        10.,
        vec![
            menu_button(),
            label(ctx, "My Account", 18.).width(110.),
            space(),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                label(ctx, "$115,000", 12.),
            ])
            .width(80.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                row(vec![label(ctx, "Operational", 12.)]),
            ])
            .width(90.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/bell.svg")).pad_y(8.5),
            ])
            .aspect_width(1.)
            .width(30.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/user.svg")).pad_y(8.5),
            ])
            .aspect_width(1.)
            .width(30.),
        ],
    )
    .pad_top(35.)
    .pad_bottom(15.)
    .pad_x(30.)
    .height(80.)
}

fn menu_button() -> Layout<'static, Drawable, ()> {
    let image = include_image!("../assets/menu-scale.svg");
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Action {
            area,
            handler: Box::new({
                let image = image.clone();
                move |ui: &mut Ui, sidebar: &mut bool, area: Area| {
                    if ui
                        .put(
                            rect_from(area),
                            Button::image(image.clone()).fill(Color32::TRANSPARENT),
                        )
                        .clicked()
                    {
                        *sidebar = !*sidebar;
                    }
                }
            }),
        }]
    })
    .aspect_width(1.)
    .width(30.)
    .height(30.)
}

fn icon(image: impl Into<ImageSource<'static>> + 'static) -> Layout<'static, Drawable, ()> {
    let image = Image::new(image).tint(Color32::WHITE);
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Action {
            area,
            handler: Box::new({
                let image = image.clone();
                move |ui: &mut Ui, _sidebar: &mut bool, area: Area| {
                    ui.put(rect_from(area), image.clone());
                }
            }),
        }]
    })
}

fn label(ctx: &egui::Context, text: &str, size: f32) -> Layout<'static, Drawable, ()> {
    label_common(ctx, text, size, false, DEMO_FG)
}

fn label_color(
    ctx: &egui::Context,
    text: &str,
    size: f32,
    color: Color32,
) -> Layout<'static, Drawable, ()> {
    label_common(ctx, text, size, false, color)
}

fn fit_label(ctx: &egui::Context, text: &str, size: f32) -> Layout<'static, Drawable, ()> {
    label_common(ctx, text, size, true, DEMO_FG)
}

fn fit_label_color(
    ctx: &egui::Context,
    text: &str,
    size: f32,
    color: Color32,
) -> Layout<'static, Drawable, ()> {
    label_common(ctx, text, size, true, color)
}

fn label_common(
    ctx: &egui::Context,
    text: &str,
    size: f32,
    fit_width: bool,
    color: Color32,
) -> Layout<'static, Drawable, ()> {
    let text = Arc::new(text.to_string());
    let ctx = ctx.clone();
    if fit_width {
        let text_for_height = text.clone();
        let ctx_for_height = ctx.clone();
        let height_calc = move |width: f32, _: &mut ()| {
            ctx_for_height.fonts(|fonts| {
                fonts
                    .layout_job(make_layout_job(
                        &text_for_height,
                        size,
                        width,
                        color,
                        EguiAlign::Min,
                        EguiAlign::Min,
                    ))
                    .size()
                    .y
            })
        };
        draw(move |area: Area, _: &mut ()| {
            vec![Drawable::Action {
                area,
                handler: Box::new({
                    let text = text.clone();
                    move |ui: &mut Ui, _sidebar: &mut bool, area: Area| {
                        let job = make_layout_job(
                            &text,
                            size,
                            area.width,
                            color,
                            EguiAlign::Min,
                            EguiAlign::Min,
                        );
                        let galley = ui.fonts(|f| f.layout_job(job));
                        ui.painter().galley(
                            Pos2::new(area.x, area.y),
                            galley,
                            Color32::TRANSPARENT,
                        );
                    }
                }),
            }]
        })
        .dynamic_height(height_calc)
    } else {
        let size_vec = ctx.fonts(|fonts| {
            fonts
                .layout_job(make_layout_job(
                    &text,
                    size,
                    300.,
                    color,
                    EguiAlign::Min,
                    EguiAlign::Min,
                ))
                .size()
        });
        draw(move |area: Area, _: &mut ()| {
            vec![Drawable::Action {
                area,
                handler: Box::new({
                    let text = text.clone();
                    move |ui: &mut Ui, _sidebar: &mut bool, area: Area| {
                        let job = make_layout_job(
                            &text,
                            size,
                            300.,
                            color,
                            EguiAlign::Min,
                            EguiAlign::Min,
                        );
                        ui.put(rect_from(area), Label::new(job));
                    }
                }),
            }]
        })
        .height(size_vec.y)
        .width(size_vec.x)
    }
}

fn col_divider(color: Color32) -> Layout<'static, Drawable, ()> {
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Action {
            area,
            handler: Box::new(move |ui: &mut Ui, _sidebar: &mut bool, area: Area| {
                ui.painter().line_segment(
                    [
                        Pos2::new(area.x, area.y + (area.height * 0.5)),
                        Pos2::new(area.x + area.width, area.y + (area.height * 0.5)),
                    ],
                    Stroke::new(1., color),
                );
            }),
        }]
    })
}

fn row_divider(color: Color32) -> Layout<'static, Drawable, ()> {
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Action {
            area,
            handler: Box::new(move |ui: &mut Ui, _sidebar: &mut bool, area: Area| {
                ui.painter().line_segment(
                    [
                        Pos2::new(area.x + (area.width * 0.5), area.y),
                        Pos2::new(area.x + (area.width * 0.5), area.y + area.height),
                    ],
                    Stroke::new(1., color),
                );
            }),
        }]
    })
}

fn rect(stroke: Color32, fill: Color32, rounding: f32) -> Layout<'static, Drawable, ()> {
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Action {
            area,
            handler: Box::new(move |ui: &mut Ui, _sidebar: &mut bool, area: Area| {
                ui.painter()
                    .rect_stroke(rect_from(area), rounding, Stroke::new(1., stroke));
                ui.painter().rect_filled(rect_from(area), rounding, fill);
            }),
        }]
    })
}

fn rect_stroke(color: Color32) -> Layout<'static, Drawable, ()> {
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Action {
            area,
            handler: Box::new(move |ui, _sidebar, area: Area| {
                ui.painter()
                    .rect_stroke(rect_from(area), 5., Stroke::new(1., color));
            }),
        }]
    })
}

fn process_commands(commands: Vec<Drawable>, ui: &mut Ui, sidebar: &mut bool) {
    for command in commands {
        match command {
            Drawable::Action { area, handler } => handler(ui, sidebar, area),
        }
    }
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

fn make_layout_job(
    text: &Arc<String>,
    size: f32,
    width: f32,
    color: Color32,
    align: EguiAlign,
    halign: EguiAlign,
) -> LayoutJob {
    let mut job = LayoutJob::single_section(
        (*text).clone().to_string(),
        egui::TextFormat {
            font_id: egui::FontId::new(size, egui::FontFamily::Proportional),
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
    job.halign = halign;
    job
}
