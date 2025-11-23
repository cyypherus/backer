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
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            let mut layout = my_layout(ui);
            let commands = layout.draw(available_area);
            process_commands(ui, commands);
        });
    })
}

fn my_layout(ui: &mut Ui) -> Layout<(), Drawable> {
    column_spaced(
        10.,
        vec![
            draw_a(ui),
            row_spaced(
                10.,
                vec![
                    draw_b(ui).width_range(200.0..),
                    column_spaced(10., vec![draw_a(ui), draw_b(ui), draw_c(ui)]),
                ],
            ),
            draw_c(ui),
        ],
    )
    .pad(10.)
}

fn draw_a(ui: &mut Ui) -> Layout<(), Drawable> {
    labeled_rect(ui, "A".to_string(), Color32::BLUE)
}

fn draw_b(ui: &mut Ui) -> Layout<(), Drawable> {
    labeled_rect(ui, "B".to_string(), Color32::RED)
}

fn draw_c(ui: &mut Ui) -> Layout<(), Drawable> {
    labeled_rect(ui, "C".to_string(), Color32::GOLD)
}

fn labeled_rect(ui: &mut Ui, text: String, color: Color32) -> Layout<(), Drawable> {
    stack(vec![draw_rect(color, true), draw_label(ui, text)])
}

fn draw_label(ui: &mut Ui, text: String) -> Layout<(), Drawable> {
    let rich_text = RichText::new(text).size(10.);
    let text_rect = egui::Label::new(rich_text.clone()).layout_in_ui(ui).1.rect;
    let width = text_rect.width();
    let height = text_rect.height();
    draw(move |_, area: Area| Drawable::Label {
        area,
        text: rich_text.clone(),
    })
    .width(width)
    .height(height)
}

fn draw_rect(color: Color32, stroke: bool) -> Layout<(), Drawable> {
    draw(move |_, area: Area| {
        if stroke {
            Drawable::RectStroke {
                area,
                rounding: 5.,
                stroke: Stroke::new(3., color),
            }
        } else {
            Drawable::RectFill {
                area,
                rounding: 5.,
                color,
            }
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
