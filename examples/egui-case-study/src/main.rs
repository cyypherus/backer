#![allow(clippy::type_complexity)]

use backer::nodes::*;
use backer::{Align, Area, Layout};
use eframe::egui;
use egui::{
  Button, Color32, Frame, Image, Layout as EguiLayout, Margin, Pos2, Rect, RichText, ScrollArea,
  Stroke, StrokeKind, Ui, Vec2,
};

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    ..Default::default()
  };
  eframe::run_native(
    "My egui App",
    options,
    Box::new(|cc| {
      egui_extras::install_image_loaders(&cc.egui_ctx);
      Ok(Box::<MyApp>::default())
    }),
  )
}

struct MyApp {
  items: Vec<Item>,
  show_backer: bool,
}

struct Item {
  title: String,
  points: i32,
}

impl Default for MyApp {
  fn default() -> Self {
    MyApp {
      items: (0..30)
        .flat_map(|_| {
          vec![
            Item {
              title: "Item 1".to_string(),
              points: 6000000,
            },
            Item {
              title: "Item 2".to_string(),
              points: 6000,
            },
            Item {
              title: "Item 3".to_string(),
              points: 80,
            },
          ]
        })
        .collect(),
      show_backer: true,
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

fn rect(area: Area) -> Rect {
  Rect {
    min: Pos2::new(area.x, area.y),
    max: Pos2::new(area.x + area.width, area.y + area.height),
  }
}

enum Drawable {
  Action {
    area: Area,
    handler: Box<dyn Fn(&mut CommandState, Area) + 'static>,
  },
}

struct CommandState<'a> {
  ui: &'a mut Ui,
  backer_on: &'a mut bool,
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      let viewport = ctx.input(|i| i.content_rect());
      if self.show_backer {
        ScrollArea::vertical().show_viewport(ui, |ui, scroll_rect| {
          let mut area = area_from(scroll_rect);
          area.y = -area.y;
          area.width = viewport.width();
          let mut layout = backer_layout(ui, &self.items);
          let commands = layout.draw(area);
          let mut state = CommandState {
            ui,
            backer_on: &mut self.show_backer,
          };
          process_commands(commands, &mut state);
        });
      } else {
        ScrollArea::vertical().show(ui, |ui| {
          ui.vertical_centered_justified(|ui| {
            if ui.button("Enable Backer").clicked() {
              self.show_backer = true
            }
            let bounties = &self.items;
            for bounty in bounties.iter() {
              Frame::group(ui.style())
                .corner_radius(10.)
                .outer_margin(Margin::same(3))
                .show(ui, |ui| {
                  ui.set_width(ui.available_width());
                  ui.horizontal(|ui| {
                    ui.add(
                      Image::new(egui::include_image!("../frs.png"))
                        .show_loading_spinner(true)
                        .fit_to_exact_size(egui::Vec2::new(45., 45.))
                        .corner_radius(4.),
                    );
                    ui.vertical(|ui| {
                      ui.add_space(5.);
                      ui.horizontal(|ui| {
                        ui.label(
                          RichText::new(bounty.title.as_str())
                            .color(Color32::WHITE)
                            .size(18.),
                        );
                        ui.label(
                          RichText::new(format!("{}XP", bounty.points)).color(Color32::WHITE),
                        );
                      });
                      ui.horizontal(|ui| {
                        ui.add_space(5.);
                        ui.label(
                          RichText::new("EXPIRES IN: 3h 2m")
                            .color(Color32::from_rgb(200, 200, 200))
                            .size(10.),
                        );
                      });
                    });
                    ui.with_layout(EguiLayout::right_to_left(egui::Align::Center), |ui| {
                      if ui
                        .add(
                          Button::new(RichText::new("Open").color(Color32::WHITE))
                            .fill(Color32::from_rgb(150, 0, 150))
                            .min_size(Vec2::new(45., 45.))
                            .corner_radius(4.),
                        )
                        .clicked()
                      {
                        dbg!("Click");
                      }
                    });
                  });
                });
            }
            ui.add_space(5.);
          });
        });
      }
    });
  }
}

fn draw_label(ui: &mut Ui, text: RichText) -> Layout<Drawable> {
  let galley = egui::Label::new(text.clone()).layout_in_ui(ui).1.rect;
  let width = galley.width();
  let height = galley.height();
  draw(move |area: Area| Drawable::Action {
    area,
    handler: Box::new({
      let text = text.clone();
      move |state: &mut CommandState, area: Area| {
        state.ui.put(rect(area), egui::Label::new(text.clone()));
      }
    }),
  })
  .width(width)
  .height(height)
}

fn backer_layout(ui: &mut Ui, items: &[Item]) -> Layout<Drawable> {
  let mut elements = Vec::with_capacity(items.len() + 1);
  elements.push(backer_toggle_button());
  elements.extend(items.iter().map(|item| bounty_card(ui, item)));
  column_spaced(10., elements).pad(10.).align(Align::Top)
}

fn backer_toggle_button() -> Layout<Drawable> {
  draw(move |area: Area| Drawable::Action {
    area,
    handler: Box::new(move |state: &mut CommandState, area: Area| {
      if state
        .ui
        .put(
          rect(area),
          Button::new("Disable Backer").min_size(Vec2::new(area.width, area.height)),
        )
        .clicked()
      {
        *state.backer_on = false;
      }
    }),
  })
  .height(15.)
}

fn bounty_card(ui: &mut Ui, item: &Item) -> Layout<Drawable> {
  let title = RichText::new(item.title.clone())
    .color(Color32::WHITE)
    .size(18.);
  let points = RichText::new(format!("{}XP", item.points)).color(Color32::WHITE);
  let expires = RichText::new("EXPIRES IN: 3h 2m")
    .color(Color32::from_rgb(200, 200, 200))
    .size(10.);

  stack(vec![
    card_outline(),
    row_spaced(
      10.,
      vec![
        bounty_image().aspect_width(1.),
        column_spaced_aligned(
          3.,
          Align::Leading,
          vec![
            row_spaced(
              10.,
              vec![
                draw_label(ui, title.clone()).align(Align::Leading),
                draw_label(ui, points.clone()),
              ],
            ),
            draw_label(ui, expires.clone())
              .align(Align::Leading)
              .pad_leading(3.),
          ],
        ),
        space(),
        open_button().aspect_width(1.),
      ],
    )
    .pad(7.),
  ])
  .height(58.)
}

fn card_outline() -> Layout<Drawable> {
  draw(move |area: Area| Drawable::Action {
    area,
    handler: Box::new(move |state: &mut CommandState, area: Area| {
      state.ui.painter().rect_stroke(
        rect(area),
        10.,
        Stroke::new(2., Color32::from_rgb(50, 50, 50)),
        StrokeKind::Middle,
      );
    }),
  })
}

fn bounty_image() -> Layout<Drawable> {
  draw(move |area: Area| Drawable::Action {
    area,
    handler: Box::new(move |state: &mut CommandState, area: Area| {
      state.ui.put(
        rect(area),
        Image::new(egui::include_image!("../frs.png"))
          .show_loading_spinner(true)
          .fit_to_exact_size(Vec2::new(area.width, area.height))
          .corner_radius(4.),
      );
    }),
  })
}

fn open_button() -> Layout<Drawable> {
  draw(move |area: Area| Drawable::Action {
    area,
    handler: Box::new(move |state: &mut CommandState, area: Area| {
      if state
        .ui
        .put(
          rect(area),
          Button::new(RichText::new("Open").color(Color32::WHITE))
            .fill(Color32::from_rgb(150, 0, 150))
            .min_size(Vec2::new(area.width, area.height))
            .corner_radius(4.),
        )
        .clicked()
      {
        dbg!("Click");
      }
    }),
  })
}

fn process_commands(commands: Vec<Drawable>, state: &mut CommandState) {
  for command in commands {
    match command {
      Drawable::Action { area, handler } => handler(state, area),
    }
  }
}
