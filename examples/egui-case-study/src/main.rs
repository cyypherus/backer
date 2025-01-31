use backer::{
  models::{Align, Area},
  nodes::*,
  Layout, Node,
};
use eframe::egui;
use egui::{
  Button, Color32, Frame, Image, Layout as EguiLayout, Margin, Pos2, Rect, RichText, ScrollArea,
  Stroke, Ui, Vec2,
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

struct State<'a> {
  ui: &'a mut Ui,
  bounties: &'a mut Vec<Item>,
  backer_on: &'a mut bool,
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      let viewport = ctx.input(|i| i.screen_rect());
      if self.show_backer {
        ScrollArea::vertical().show_viewport(ui, |ui, scroll_rect| {
          let mut state = State {
            ui,
            bounties: &mut self.items,
            backer_on: &mut self.show_backer,
          };
          let mut area = area_from(scroll_rect);
          area.y = -area.y;
          area.width = viewport.width();
          Layout::new(dynamic(|state: &mut State| {
            column_spaced(
              10.,
              vec![
                draw(|area, state: &mut State| {
                  if state
                    .ui
                    .put(rect(area), Button::new("Backer Off"))
                    .clicked()
                  {
                    *state.backer_on = false
                  }
                })
                .height(15.),
                group(
                  state
                    .bounties
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                      stack(vec![
                        draw(|area, state: &mut State| {
                          state.ui.painter().rect_stroke(
                            rect(area),
                            10.,
                            Stroke::new(2., Color32::from_rgb(50, 50, 50)),
                          );
                        }),
                        row_spaced(
                          10.,
                          vec![
                            draw(|area, state: &mut State| {
                              state.ui.put(
                                rect(area),
                                Image::new(egui::include_image!("../frs.png"))
                                  .show_loading_spinner(true)
                                  .fit_to_exact_size(egui::Vec2::new(area.width, area.height))
                                  .rounding(4.),
                              );
                            })
                            .aspect(1.),
                            column_spaced(
                              3.,
                              vec![
                                row_spaced(
                                  10.,
                                  vec![
                                    draw_label(
                                      state.ui,
                                      RichText::new(state.bounties[i].title.as_str())
                                        .color(Color32::WHITE)
                                        .size(18.),
                                    )
                                    .align(Align::Leading),
                                    draw_label(
                                      state.ui,
                                      RichText::new(format!("{}XP", item.points))
                                        .color(Color32::WHITE),
                                    ),
                                  ],
                                ),
                                draw_label(
                                  state.ui,
                                  RichText::new("EXPIRES IN: 3h 2m")
                                    .color(Color32::from_rgb(200, 200, 200))
                                    .size(10.),
                                )
                                .align(Align::Leading)
                                .pad_leading(3.),
                              ],
                            )
                            .align_contents(Align::Leading)
                            .width_range(120.0..),
                            space(),
                            draw(|area, state: &mut State| {
                              if state
                                .ui
                                .put(
                                  rect(area),
                                  Button::new(RichText::new("Open").color(Color32::WHITE))
                                    .fill(Color32::from_rgb(150, 0, 150))
                                    .rounding(4.),
                                )
                                .clicked()
                              {
                                dbg!("Click");
                              }
                            })
                            .aspect(1.),
                          ],
                        )
                        .pad(7.),
                      ])
                      .height(58.)
                    })
                    .collect(),
                ),
              ],
            )
            .align_contents(Align::Top)
            .pad(10.)
            .align(Align::Top)
          }))
          .draw(area, &mut state);
        });
      } else {
        ScrollArea::vertical().show(ui, |ui| {
          ui.vertical_centered_justified(|ui| {
            if ui.button("Backer On").clicked() {
              self.show_backer = true
            }
            let bounties = &self.items;
            for bounty in bounties.iter() {
              Frame::group(ui.style())
                .rounding(10.)
                .outer_margin(Margin::same(3.))
                .show(ui, |ui| {
                  ui.set_width(ui.available_width());
                  ui.horizontal(|ui| {
                    ui.add(
                      Image::new(egui::include_image!("../frs.png"))
                        .show_loading_spinner(true)
                        .fit_to_exact_size(egui::Vec2::new(45., 45.))
                        .rounding(4.),
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
                            .rounding(4.),
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

fn draw_label<'a>(ui: &'_ mut Ui, text: RichText) -> Node<'a, State<'a>> {
  let label = egui::Label::new(text.clone());
  let galley = label.layout_in_ui(ui).1.rect;
  let text_area = area_from(galley);
  draw(move |area, state: &mut State| {
    state.ui.put(rect(area), egui::Label::new(text.clone()));
  })
  .width(text_area.width)
  .height(text_area.height)
}
