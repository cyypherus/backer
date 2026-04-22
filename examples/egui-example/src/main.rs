use backer::nodes::*;
use backer::{Area, Layout};
use eframe::egui;

use egui::{Color32, Pos2, Rect, RichText, Stroke, StrokeKind, Ui};

enum Drawable {
    RectStroke {
        area: Area,
        rounding: f32,
        stroke: Stroke,
    },
    RectFill {
        area: Area,
        rounding: f32,
        color: Color32,
    },
    Label {
        area: Area,
        text: RichText,
    },
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Layout Example", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let viewport = ctx.input(|i| i.content_rect());
            let available_area = area_from(viewport);
            let mut layout = my_layout(ctx);
            let commands = layout.draw(available_area, &mut ());
            process_commands(ui, commands);
        });
    })
}

fn my_layout(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    column_spaced(
        10.,
        vec![
            draw_a(ctx),
            row_spaced(
                10.,
                vec![
                    draw_b(ctx).width_range(200.0..),
                    column_spaced(10., vec![draw_a(ctx), draw_b(ctx), draw_c(ctx)]),
                ],
            ),
            draw_c(ctx),
        ],
    )
    .pad(10.)
}

fn draw_a(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    labeled_rect(ctx, "A".to_string(), Color32::BLUE)
}

fn draw_b(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    labeled_rect(ctx, "B".to_string(), Color32::RED)
}

fn draw_c(ctx: &egui::Context) -> Layout<'static, Drawable, ()> {
    labeled_rect(ctx, "C".to_string(), Color32::GOLD)
}

fn labeled_rect(
    ctx: &egui::Context,
    text: String,
    color: Color32,
) -> Layout<'static, Drawable, ()> {
    stack(vec![draw_rect(color, true), draw_label(ctx, text)])
}

fn draw_label(ctx: &egui::Context, text: String) -> Layout<'static, Drawable, ()> {
    let rich_text = RichText::new(text).size(10.);
    let job = egui::text::LayoutJob::simple_singleline(
        rich_text.text().to_string(),
        egui::FontId::proportional(10.),
        Color32::WHITE,
    );
    let galley = ctx.fonts_mut(|f| f.layout_job(job));
    let width = galley.size().x;
    let height = galley.size().y;
    draw(move |area: Area, _: &mut ()| {
        vec![Drawable::Label {
            area,
            text: rich_text.clone(),
        }]
    })
    .width(width)
    .height(height)
}

fn draw_rect(color: Color32, stroke: bool) -> Layout<'static, Drawable, ()> {
    draw(move |area: Area, _: &mut ()| {
        if stroke {
            vec![Drawable::RectStroke {
                area,
                rounding: 5.,
                stroke: Stroke::new(3., color),
            }]
        } else {
            vec![Drawable::RectFill {
                area,
                rounding: 5.,
                color,
            }]
        }
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

fn rect(area: Area) -> Rect {
    Rect {
        min: Pos2::new(area.x, area.y),
        max: Pos2::new(area.x + area.width, area.y + area.height),
    }
}

fn process_commands(ui: &mut Ui, commands: Vec<Drawable>) {
    for command in commands {
        match command {
            Drawable::RectStroke {
                area,
                rounding,
                stroke,
            } => {
                ui.painter()
                    .rect_stroke(rect(area), rounding, stroke, StrokeKind::Middle);
            }
            Drawable::RectFill {
                area,
                rounding,
                color,
            } => {
                ui.painter().rect_filled(rect(area), rounding, color);
            }
            Drawable::Label { area, text } => {
                ui.put(rect(area), egui::Label::new(text));
            }
        }
    }
}
