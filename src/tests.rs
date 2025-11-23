#[cfg(test)]
mod tests_module {
    use crate::nodes::*;
    use crate::*;

    macro_rules! assert_area {
        ($expected:expr) => {
            draw(move |a| assert_eq!(a, $expected))
        };
    }

    // RULES
    // Containers (stack, row, column) are only as big as their children require via constraints, unless explicitly marked for expansion using `.expand`
    // Non-containers (draw, space) take as much space as is available, unless they are constrained to specific dimensions
    // Sequences (row, column) distribute space to immediate constrained children according to their constraints, & split the remainder among the rest of their children.
    // The alignment of a node defines how it will be placed when there is less *or* more space available than it requires along a given axis.

    #[test]
    fn test_containers_hug_children_column() {
        column(vec![
            assert_area!(Area::new(45., 40., 10., 10.))
                .width(10.)
                .height(10.),
            assert_area!(Area::new(45., 50., 10., 10.))
                .width(10.)
                .height(10.),
        ])
        .attach_under(assert_area!(Area::new(45., 40., 10., 20.)))
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_containers_hug_children_row() {
        row(vec![
            assert_area!(Area::new(40., 45., 10., 10.))
                .width(10.)
                .height(10.),
            assert_area!(Area::new(50., 45., 10., 10.))
                .width(10.)
                .height(10.),
        ])
        .attach_under(assert_area!(Area::new(40., 45., 20., 10.)))
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_containers_hug_children_stack() {
        stack(vec![
            assert_area!(Area::new(49., 49., 2., 2.))
                .width(2.)
                .height(2.),
            assert_area!(Area::new(45., 45., 10., 10.))
                .width(10.)
                .height(10.),
        ])
        .attach_under(assert_area!(Area::new(45., 45., 10., 10.)))
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_containers_hug_children_column_nested() {
        column(vec![
            column(vec![
                assert_area!(Area::new(45., 40., 10., 10.))
                    .width(10.)
                    .height(10.),
            ])
            .attach_under(assert_area!(Area::new(45., 40., 10., 10.))),
            row(vec![
                assert_area!(Area::new(45., 50., 10., 10.))
                    .width(10.)
                    .height(10.),
            ])
            .attach_under(assert_area!(Area::new(45., 50., 10., 10.))),
        ])
        .attach_under(assert_area!(Area::new(45., 40., 10., 20.)))
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    impl<A> Layout<A> {
        fn debug_visualize(&mut self, available_area: Area) {
            fn visualize_areas(areas: &[Area], bounds: Area) {
                if areas.is_empty() {
                    return;
                }

                let scale_x = 0.5;
                let scale_y = 0.18;
                let grid_width = (bounds.width * scale_x).ceil() as usize;
                let grid_height = (bounds.height * scale_y).ceil() as usize;

                let mut grid = vec![vec![' '; grid_width]; grid_height];

                // Draw border around bounds
                draw_border(&mut grid);

                for (i, area) in areas.iter().enumerate() {
                    let char_to_use = char::from_digit((i % 10) as u32, 10).unwrap_or('*');
                    draw_box(&mut grid, *area, bounds, scale_x, scale_y, char_to_use);
                }

                println!("{}", grid_to_ascii(&grid));
            }

            fn draw_border(grid: &mut [Vec<char>]) {
                if grid.is_empty() || grid[0].is_empty() {
                    return;
                }

                let height = grid.len();
                let width = grid[0].len();

                // Top and bottom borders
                for x in 0..width {
                    grid[0][x] = '─';
                    if height > 1 {
                        grid[height - 1][x] = '─';
                    }
                }

                // Left and right borders
                for row in grid.iter_mut() {
                    row[0] = '│';
                    if width > 1 {
                        row[width - 1] = '│';
                    }
                }

                // Corners
                if width > 0 && height > 0 {
                    grid[0][0] = '┌';
                    if width > 1 {
                        grid[0][width - 1] = '┐';
                    }
                    if height > 1 {
                        grid[height - 1][0] = '└';
                        if width > 1 {
                            grid[height - 1][width - 1] = '┘';
                        }
                    }
                }
            }
            fn draw_box(
                grid: &mut [Vec<char>],
                area: Area,
                bounds: Area,
                scale_x: f32,
                scale_y: f32,
                ch: char,
            ) {
                let start_x = ((area.x - bounds.x) * scale_x).max(0.0) as usize;
                let start_y = ((area.y - bounds.y) * scale_y).max(0.0) as usize;
                let end_x =
                    ((area.x + area.width - bounds.x) * scale_x).min(grid[0].len() as f32) as usize;
                let end_y =
                    ((area.y + area.height - bounds.y) * scale_y).min(grid.len() as f32) as usize;

                if start_x >= end_x || start_y >= end_y {
                    return;
                }

                // Fill interior
                for y in (start_y + 1)..end_y.saturating_sub(1) {
                    for x in (start_x + 1)..end_x.saturating_sub(1) {
                        if y < grid.len() && x < grid[0].len() {
                            grid[y][x] = ch;
                        }
                    }
                }

                // Draw box borders
                for x in start_x..end_x {
                    if start_y < grid.len() && x < grid[0].len() {
                        grid[start_y][x] = '─';
                    }
                    if end_y > 0 && end_y - 1 < grid.len() && x < grid[0].len() {
                        grid[end_y - 1][x] = '─';
                    }
                }

                for y in start_y..end_y {
                    if y < grid.len() && start_x < grid[0].len() {
                        grid[y][start_x] = '│';
                    }
                    if y < grid.len() && end_x > 0 && end_x - 1 < grid[0].len() {
                        grid[y][end_x - 1] = '│';
                    }
                }

                // Box corners
                if start_y < grid.len() && start_x < grid[0].len() {
                    grid[start_y][start_x] = '┌';
                }
                if start_y < grid.len() && end_x > 0 && end_x - 1 < grid[0].len() {
                    grid[start_y][end_x - 1] = '┐';
                }
                if end_y > 0 && end_y - 1 < grid.len() && start_x < grid[0].len() {
                    grid[end_y - 1][start_x] = '└';
                }
                if end_y > 0 && end_y - 1 < grid.len() && end_x > 0 && end_x - 1 < grid[0].len() {
                    grid[end_y - 1][end_x - 1] = '┘';
                }
            }

            fn grid_to_ascii(grid: &[Vec<char>]) -> String {
                let mut result = String::new();

                for row in grid {
                    result.extend(row.iter());
                    result.push('\n');
                }

                result
            }
            let mut area_layout = self.to_area_layout();

            visualize_areas(
                &area_layout
                    .draw(available_area)
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>(),
                Area::new(0., 0., 100., 100.),
            );
        }

        fn to_area_layout(&self) -> Layout<Area> {
            use crate::types::LayoutType;

            fn transform_node<A>(node: &Layout<A>) -> Layout<Area> {
                let new_layout = match &node.layout {
                    LayoutType::Draw(_) => LayoutType::Draw(Some(Box::new(|area| area))),
                    LayoutType::Column {
                        spacing,
                        x_align,
                        y_align,
                    } => LayoutType::Column {
                        spacing: *spacing,
                        x_align: *x_align,
                        y_align: *y_align,
                    },
                    LayoutType::Row {
                        spacing,
                        x_align,
                        y_align,
                    } => LayoutType::Row {
                        spacing: *spacing,
                        x_align: *x_align,
                        y_align: *y_align,
                    },
                    LayoutType::Stack { x_align, y_align } => LayoutType::Stack {
                        x_align: *x_align,
                        y_align: *y_align,
                    },
                    LayoutType::Padding {
                        leading,
                        trailing,
                        top,
                        bottom,
                    } => LayoutType::Padding {
                        leading: *leading,
                        trailing: *trailing,
                        top: *top,
                        bottom: *bottom,
                    },
                    LayoutType::Offset { x, y } => LayoutType::Offset { x: *x, y: *y },
                    LayoutType::Space => LayoutType::Space,
                    LayoutType::Empty => LayoutType::Empty,
                    LayoutType::Coupled { over } => LayoutType::Coupled { over: *over },
                    LayoutType::AreaReader { .. } => LayoutType::AreaReader { func: None },
                };

                Layout {
                    layout: new_layout,
                    constraints: node.constraints,
                    layer: None,
                    dynamic_constraints: Default::default(),
                    resolved: node.resolved,
                    allocated: node.allocated,
                    children: node.children.iter().map(transform_node).collect(),
                }
            }

            transform_node(self)
        }
    }

    #[test]
    fn test_expands_nested_nodes() {
        let values =
            area_reader(|_| area_reader(|_| draw(|_| 1))).draw(Area::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], 1);
    }

    #[test]
    fn test_draws_all_expanded_nodes() {
        let values = area_reader(|_| {
            stack(vec![
                stack(vec![area_reader(|_| draw(|_| 1)), draw(|_| 1)]),
                draw(|_| 1),
            ])
        })
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_simple_column_layout() {
        column(vec![
            draw(|a| {
                assert_eq!(a.width, 100.0);
                assert_eq!(a.height, 50.0);
            })
            .height(50.0),
            draw(|a| {
                assert_eq!(a.width, 100.0);
                assert_eq!(a.height, 50.0);
                assert_eq!(a.y, 50.0);
            })
            .height(50.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_simple_row_layout() {
        row(vec![
            draw(|a| {
                assert_eq!(a.width, 50.0);
                assert_eq!(a.height, 100.0);
            })
            .width(50.0),
            draw(|a| {
                assert_eq!(a.width, 50.0);
                assert_eq!(a.height, 100.0);
                assert_eq!(a.x, 50.0);
            })
            .width(50.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_nested_layout() {
        column(vec![
            row(vec![
                draw(|a| {
                    assert_eq!(a.width, 50.0);
                    assert_eq!(a.height, 25.0);
                })
                .width(50.0),
                draw(|a| {
                    assert_eq!(a.width, 50.0);
                    assert_eq!(a.height, 25.0);
                    assert_eq!(a.x, 50.0);
                })
                .width(50.0),
            ])
            .height(25.0),
            draw(|a| {
                assert_eq!(a.width, 100.0);
                assert_eq!(a.height, 75.0);
                assert_eq!(a.y, 25.0);
            })
            .height(75.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_padding() {
        draw(|a| {
            assert_eq!(a.x, 10.0);
            assert_eq!(a.y, 10.0);
            assert_eq!(a.width, 80.0);
            assert_eq!(a.height, 80.0);
        })
        .pad(10.0)
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_stack_layout() {
        let values = stack(vec![
            draw(|a| {
                assert_eq!(a.width, 100.0);
                assert_eq!(a.height, 100.0);
                1
            }),
            draw(|a| {
                assert_eq!(a.width, 100.0);
                assert_eq!(a.height, 100.0);
                1
            }),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_dynamic_node() {
        draw(|a| {
            assert_eq!(a.width, 100.0);
        })
        .height(50.0)
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_dynamic_node_drawing_issue() {
        let values = column(vec![
            draw(|_| "dynamic_child_1".to_string()).height(20.0),
            draw(|_| "static_draw".to_string()).height(30.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));

        println!("Draw calls: {:?}", values);
        assert!(
            !values.is_empty(),
            "Dynamic node children should have been drawn"
        );
        assert!(
            values.contains(&"static_draw".to_string()),
            "Static draw should be called"
        );
    }

    #[test]
    fn test_nested_dynamic_nodes() {
        let values = column(vec![
            draw(|_| "outer_before".to_string()).height(10.0),
            draw(|_| "inner_1".to_string()).height(20.0),
            draw(|_| "outer_after".to_string()).height(15.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));

        println!("Nested draw calls: {:?}", values);
        assert!(
            values.contains(&"outer_before".to_string()),
            "Outer before should be drawn"
        );
        assert!(
            values.contains(&"outer_after".to_string()),
            "Outer after should be drawn"
        );
        assert!(
            values.iter().any(|call| call.starts_with("inner_")),
            "Inner dynamic should be drawn"
        );
    }

    #[test]
    fn test_row_dynamic() {
        column(vec![
            row(vec![
                space().height(0.),
                assert_area!(Area::new(50., 0., 50., 20.)).height(20.),
            ]),
            assert_area!(Area::new(0., 20., 100., 80.)),
        ])
        .draw(Area::new(0., 0., 100., 100.));
    }

    #[test]
    fn test_static_vs_dynamic_height() {
        column(vec![
            row(vec![
                space().height(0.),
                assert_area!(Area::new(50., 0., 50., 30.)).height(30.),
            ]),
            assert_area!(Area::new(0., 30., 100., 70.)),
        ])
        .draw(Area::new(0., 0., 100., 100.));

        column(vec![
            row(vec![
                space().height(0.),
                assert_area!(Area::new(50., 0., 50., 30.)).height(30.),
            ]),
            assert_area!(Area::new(0., 30., 100., 70.)),
        ])
        .draw(Area::new(0., 0., 100., 100.));
    }

    #[cfg(test)]
    mod layout_tests {

        use super::*;
        #[test]
        fn test_seq_align_on_axis() {
            row_aligned(
                Align::Leading,
                vec![
                    assert_area!(Area::new(0., 0., 10., 100.)).width(10.),
                    assert_area!(Area::new(10., 0., 30., 100.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                assert_area!(Area::new(30., 0., 10., 100.)).width(10.),
                assert_area!(Area::new(40., 0., 30., 100.)).width(30.),
            ])
            .align(Align::CenterX)
            .draw(Area::new(0., 0., 100., 100.));

            row_aligned(
                Align::Trailing,
                vec![
                    assert_area!(Area::new(60., 0., 10., 100.)).width(10.),
                    assert_area!(Area::new(70., 0., 30., 100.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            column_aligned(
                Align::Top,
                vec![
                    assert_area!(Area::new(0., 0., 100., 10.)).height(10.),
                    assert_area!(Area::new(0., 10., 100., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                assert_area!(Area::new(0., 30., 100., 10.)).height(10.),
                assert_area!(Area::new(0., 40., 100., 30.)).height(30.),
            ])
            .align(Align::CenterY)
            .draw(Area::new(0., 0., 100., 100.));

            column_aligned(
                Align::Bottom,
                vec![
                    assert_area!(Area::new(0., 60., 100., 10.)).height(10.),
                    assert_area!(Area::new(0., 70., 100., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_seq_align_off_axis() {
            column_aligned(
                Align::Leading,
                vec![
                    assert_area!(Area::new(0., 0., 10., 50.)).width(10.),
                    assert_area!(Area::new(0., 50., 30., 50.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                assert_area!(Area::new(45., 0., 10., 50.)).width(10.),
                assert_area!(Area::new(35., 50., 30., 50.)).width(30.),
            ])
            .align(Align::CenterX)
            .draw(Area::new(0., 0., 100., 100.));

            column_aligned(
                Align::Trailing,
                vec![
                    assert_area!(Area::new(90., 0., 10., 50.)).width(10.),
                    assert_area!(Area::new(70., 50., 30., 50.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            row_aligned(
                Align::Top,
                vec![
                    assert_area!(Area::new(0., 0., 50., 10.)).height(10.),
                    assert_area!(Area::new(50., 0., 50., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                assert_area!(Area::new(0., 45., 50., 10.)).height(10.),
                assert_area!(Area::new(50., 35., 50., 30.)).height(30.),
            ])
            .align(Align::CenterY)
            .draw(Area::new(0., 0., 100., 100.));

            row_aligned(
                Align::Bottom,
                vec![
                    assert_area!(Area::new(0., 90., 50., 10.)).height(10.),
                    assert_area!(Area::new(50., 70., 50., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_seq_align_on_axis_nested_seq() {
            row_aligned(
                Align::Leading,
                vec![
                    row(vec![assert_area!(Area::new(0., 0., 10., 100.)).width(10.)]),
                    assert_area!(Area::new(10., 0., 30., 100.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                row(vec![assert_area!(Area::new(30., 0., 10., 100.)).width(10.)]),
                assert_area!(Area::new(40., 0., 30., 100.)).width(30.),
            ])
            .align(Align::CenterX)
            .draw(Area::new(0., 0., 100., 100.));

            row_aligned(
                Align::Trailing,
                vec![
                    row(vec![assert_area!(Area::new(60., 0., 10., 100.)).width(10.)]),
                    assert_area!(Area::new(70., 0., 30., 100.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            column_aligned(
                Align::Top,
                vec![
                    row(vec![assert_area!(Area::new(0., 0., 100., 10.)).height(10.)]),
                    assert_area!(Area::new(0., 10., 100., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
            column(vec![
                row(vec![
                    assert_area!(Area::new(0., 30., 100., 10.)).height(10.),
                ]),
                assert_area!(Area::new(0., 40., 100., 30.)).height(30.),
            ])
            .align(Align::CenterY)
            .draw(Area::new(0., 0., 100., 100.));

            column_aligned(
                Align::Bottom,
                vec![
                    row(vec![
                        assert_area!(Area::new(0., 60., 100., 10.)).height(10.),
                    ]),
                    assert_area!(Area::new(0., 70., 100., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_seq_align_off_axis_nested_seq() {
            column_aligned(
                Align::Leading,
                vec![
                    row(vec![assert_area!(Area::new(0., 0., 10., 50.)).width(10.)]),
                    assert_area!(Area::new(0., 50., 30., 50.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                row(vec![assert_area!(Area::new(45., 0., 10., 50.)).width(10.)]),
                assert_area!(Area::new(35., 50., 30., 50.)).width(30.),
            ])
            .align(Align::CenterX)
            .draw(Area::new(0., 0., 100., 100.));

            column_aligned(
                Align::Trailing,
                vec![
                    row(vec![assert_area!(Area::new(90., 0., 10., 50.)).width(10.)]),
                    assert_area!(Area::new(70., 50., 30., 50.)).width(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            row_aligned(
                Align::Top,
                vec![
                    row(vec![assert_area!(Area::new(0., 0., 50., 10.)).height(10.)]),
                    assert_area!(Area::new(50., 0., 50., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                row(vec![assert_area!(Area::new(0., 45., 50., 10.)).height(10.)]),
                assert_area!(Area::new(50., 35., 50., 30.)).height(30.),
            ])
            .align(Align::CenterY)
            .draw(Area::new(0., 0., 100., 100.));

            row_aligned(
                Align::Bottom,
                vec![
                    row(vec![assert_area!(Area::new(0., 90., 50., 10.)).height(10.)]),
                    assert_area!(Area::new(50., 70., 50., 30.)).height(30.),
                ],
            )
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_aspect_ratio() {
            assert_area!(Area::new(0., 0., 100., 100.))
                .aspect_width(1.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(25., 0., 50., 100.))
                .aspect_width(0.5)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 0., 50., 100.))
                .aspect_width(0.5)
                .align(Align::Leading)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(50., 0., 50., 100.))
                .aspect_width(0.5)
                .align(Align::Trailing)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 25., 100., 50.))
                .aspect_height(2.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 0., 100., 50.))
                .aspect_height(2.)
                .align(Align::Top)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 50., 100., 50.))
                .aspect_height(2.)
                .align(Align::Bottom)
                .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_aspect_ratio_in_seq() {
            row(vec![
                assert_area!(Area::new(0., 0., 100., 100.)).aspect_width(1.),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            stack(vec![
                assert_area!(Area::new(25., 0., 50., 100.)).aspect_width(0.5),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                assert_area!(Area::new(0., 0., 50., 100.))
                    .aspect_width(0.5)
                    .align(Align::Leading),
            ])
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            stack(vec![
                assert_area!(Area::new(50., 0., 50., 100.))
                    .aspect_width(0.5)
                    .align(Align::Trailing),
            ])
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_aspect_ratio_nested() {
            column(vec![
                assert_area!(Area::new(0., 0., 200., 50.)),
                row(vec![
                    assert_area!(Area::new(0., 50., 150., 50.)),
                    assert_area!(Area::new(150., 50., 50., 50.)).aspect_width(1.),
                ]),
            ])
            .draw(Area::new(0., 0., 200., 100.));
        }

        #[test]
        fn test_pad() {
            assert_area!(Area::new(10., 10., 80., 80.))
                .pad(10.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(10., 0., 80., 100.))
                .pad_x(10.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 10., 100., 80.))
                .pad_y(10.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(10., 0., 90., 100.))
                .pad_leading(10.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 0., 90., 100.))
                .pad_trailing(10.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 10., 100., 90.))
                .pad_top(10.)
                .draw(Area::new(0., 0., 100., 100.));

            assert_area!(Area::new(0., 0., 100., 90.))
                .pad_bottom(10.)
                .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_aspect_ratio_in_pad() {
            assert_area!(Area::new(25., 0., 50., 100.))
                .aspect_width(0.5)
                .draw(Area::new(0., 0., 100., 100.));

            stack(vec![
                assert_area!(Area::new(30., 10., 40., 80.))
                    .aspect_width(0.5)
                    .pad(10.),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            stack(vec![
                assert_area!(Area::new(35., 10., 30., 80.))
                    .pad(10.)
                    .aspect_width(0.5),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_aspect_ratio_fit() {
            column(vec![
                assert_area!(Area::new(0., 0., 100., 50.)),
                assert_area!(Area::new(25., 50., 50., 50.)).aspect_width(1.),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                assert_area!(Area::new(25., 0., 50., 50.)).aspect_width(1.),
                assert_area!(Area::new(25., 50., 50., 50.)).aspect_width(1.),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                assert_area!(Area::new(0., 0., 50., 100.)),
                assert_area!(Area::new(50., 25., 50., 50.)).aspect_height(1.),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                assert_area!(Area::new(0., 25., 50., 50.)).aspect_height(1.),
                assert_area!(Area::new(50., 25., 50., 50.)).aspect_height(1.),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_space_expansion() {
            row(vec![
                assert_area!(Area::new(0., 0., 1., 100.)).width(1.),
                space(),
                assert_area!(Area::new(998., 0., 1., 100.)).width(1.),
                assert_area!(Area::new(999., 0., 1., 100.)).width(1.),
            ])
            .draw(Area::new(0., 0., 1000., 100.));
        }
        // #[test]
        // fn test_explicit_aspect() {
        //     Layout::new({
        //         column_spaced(
        //             10.,
        //             vec![
        //                 draw(|a| {
        //                     assert_eq!(a, Area::new(45., 0., 10., 20.));
        //                 })
        //                 .width(10.)
        //                 .aspect_width(0.5),
        //                 draw(|a| {
        //                     // assert_eq!(a, Area::new(0., 30., 100., 70.));
        //                 }),
        //             ],
        //         )
        //     })
        //     .debug_visualize(Area::new(0., 0., 100., 100.));
        // }
        #[test]
        fn test_explicit_with_padding() {
            column(vec![
                assert_area!(Area::new(10., 10., 80., 20.))
                    .height(20.)
                    .pad(10.),
                assert_area!(Area::new(0., 40., 100., 60.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        // #[test]
        // fn test_explicit_in_explicit() {
        //     draw(|a| {
        //         assert_eq!(a, Area::new(40., 0., 20., 100.));
        //     })
        //     .width_range(20.0..)
        //     .pad(0.)
        //     .attach_under(draw(|a| {
        //         assert_eq!(a, Area::new(40., 0., 20., 100.));
        //     }))
        //     .width_range(..10.)
        //     .attach_under(draw(|a| {
        //         assert_eq!(a, Area::new(45., 0., 10., 100.));
        //     }))
        //     .draw(Area::new(0., 0., 100., 100.));
        // }

        #[test]
        fn test_compressed_expanded_respects_lower_bound() {
            stack(vec![
                assert_area!(Area::new(0., -50., 100., 200.)).height(200.),
                assert_area!(Area::new(0., -50., 100., 200.)),
            ])
            .expand()
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                stack(vec![
                    assert_area!(Area::new(0., -50., 100., 200.)).height(200.),
                    assert_area!(Area::new(0., -50., 100., 200.)),
                ])
                .expand(),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_compressed_aspect_ratio() {
            row(vec![
                assert_area!(Area::new(0., 25., 50., 50.)).aspect_width(1.),
                assert_area!(Area::new(50., 0., 50., 100.)).width(50.),
            ])
            .attach_under(assert_area!(Area::new(0., 0., 100., 100.)))
            .debug_visualize(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_dynamic_attached() {
            row(vec![
                space(),
                assert_area!(Area::new(25., 25., 25., 50.))
                    .dynamic_height(|h| h * 2.)
                    .attach_under(assert_area!(Area::new(25., 25., 25., 50.))),
                space(),
                space(),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
    }

    #[cfg(test)]
    mod sequence_tests {
        use super::*;
        #[test]
        fn test_column_basic() {
            column(vec![
                assert_area!(Area::new(0., 0., 100., 50.)),
                assert_area!(Area::new(0., 50., 100., 50.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_column_constrained_1() {
            column(vec![
                assert_area!(Area::new(0., 0., 100., 10.)).height(10.),
                assert_area!(Area::new(0., 10., 100., 90.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                assert_area!(Area::new(0., 0., 100., 10.)).height(10.),
                assert_area!(Area::new(0., 10., 100., 90.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_column_constrained_2() {
            column(vec![
                assert_area!(Area::new(0., 0., 100., 90.)),
                assert_area!(Area::new(0., 90., 100., 10.)).height(10.),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            column(vec![
                assert_area!(Area::new(0., 0., 100., 90.)),
                assert_area!(Area::new(0., 90., 100., 10.)).height(10.),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_row_basic() {
            row(vec![
                assert_area!(Area::new(0., 0., 50., 100.)),
                assert_area!(Area::new(50., 0., 50., 100.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_row_constrained_1() {
            row(vec![
                assert_area!(Area::new(0., 25., 10., 50.))
                    .width(10.)
                    .height(50.),
                assert_area!(Area::new(10., 0., 90., 100.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                assert_area!(Area::new(0., 0., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::Top),
                assert_area!(Area::new(10., 40., 10., 20.))
                    .width(10.)
                    .height(20.),
                assert_area!(Area::new(20., 80., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::Bottom),
                assert_area!(Area::new(30., 0., 70., 100.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }
        #[test]
        fn test_row_constrained_2() {
            row(vec![
                assert_area!(Area::new(0., 0., 70., 100.)),
                assert_area!(Area::new(70., 0., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::Top),
                assert_area!(Area::new(80., 40., 10., 20.))
                    .width(10.)
                    .height(20.),
                assert_area!(Area::new(90., 80., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::Bottom),
            ])
            .draw(Area::new(0., 0., 100., 100.));

            row(vec![
                assert_area!(Area::new(0., 0., 70., 100.)),
                assert_area!(Area::new(70., 0., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::Top),
                assert_area!(Area::new(80., 40., 10., 20.))
                    .width(10.)
                    .height(20.),
                assert_area!(Area::new(90., 80., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::Bottom),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_stack_basic() {
            stack(vec![
                assert_area!(Area::new(0., 0., 100., 100.)),
                assert_area!(Area::new(0., 0., 100., 100.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_stack_alignment() {
            stack(vec![
                assert_area!(Area::new(0., 0., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::TopLeading),
                assert_area!(Area::new(45., 0., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::TopCenter),
                assert_area!(Area::new(90., 0., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::TopTrailing),
                assert_area!(Area::new(90., 40., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::CenterTrailing),
                assert_area!(Area::new(90., 80., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::BottomTrailing),
                assert_area!(Area::new(45., 80., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::BottomCenter),
                assert_area!(Area::new(0., 80., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::BottomLeading),
                assert_area!(Area::new(0., 40., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::CenterLeading),
                assert_area!(Area::new(45., 40., 10., 20.))
                    .width(10.)
                    .height(20.)
                    .align(Align::CenterCenter),
            ])
            .expand()
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_sequence_spacing() {
            row_spaced(
                10.,
                vec![
                    assert_area!(Area::new(0., 40., 10., 20.))
                        .width(10.)
                        .height(20.),
                    assert_area!(Area::new(20., 0., 25., 100.)),
                    assert_area!(Area::new(55., 40., 10., 20.))
                        .width(10.)
                        .height(20.),
                    assert_area!(Area::new(75., 0., 25., 100.)),
                ],
            )
            .draw(Area::new(0., 0., 100., 100.));

            column_spaced(
                10.,
                vec![
                    assert_area!(Area::new(0., 0., 100., 15.)),
                    assert_area!(Area::new(45., 25., 10., 20.))
                        .width(10.)
                        .height(20.),
                    assert_area!(Area::new(0., 55., 100., 15.)),
                    assert_area!(Area::new(45., 80., 10., 20.))
                        .width(10.)
                        .height(20.),
                ],
            )
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_row_with_constrained_item() {
            row(vec![
                assert_area!(Area::new(0., 0., 30., 100.)).width(30.),
                assert_area!(Area::new(30., 0., 70., 100.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_nested_row_with_constrained_item() {
            row(vec![
                row(vec![
                    assert_area!(Area::new(0., 0., 20., 100.)).width(20.),
                    assert_area!(Area::new(20., 0., 30., 100.)),
                ])
                .width(50.),
                assert_area!(Area::new(50., 0., 50., 100.)),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_stack_with_constrained_item() {
            stack(vec![
                assert_area!(Area::new(0., 0., 100., 100.)),
                assert_area!(Area::new(25., 25., 50., 50.))
                    .width(50.)
                    .height(50.),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_row_with_multiple_constrained_items() {
            row(vec![
                assert_area!(Area::new(0., 0., 20., 100.)).width(20.),
                assert_area!(Area::new(20., 0., 30., 100.)).width(30.),
                draw(|a| {
                    assert!((a.x - 50.0).abs() < 0.001);
                    assert!((a.y - 0.0).abs() < 0.001);
                    assert!((a.width - 50.0).abs() < 0.001);
                    assert!((a.height - 100.0).abs() < 0.001);
                }),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_row_with_constrained_height_in_column() {
            column(vec![
                draw(|a| {
                    // Should get 40px height (half of remaining 80px after row takes 20px)
                    assert_eq!(a, Area::new(0., 0., 100., 40.));
                }),
                row(vec![
                    draw(|a| {
                        // Row content should be 20px tall
                        assert_eq!(a, Area::new(0., 40., 50., 20.));
                    })
                    .height(20.),
                    draw(|a| {
                        // Row content should be 20px tall
                        assert_eq!(a, Area::new(50., 40., 50., 20.));
                    })
                    .height(20.),
                ]),
                draw(|a| {
                    // Should get 40px height (half of remaining 80px after row takes 20px)
                    assert_eq!(a, Area::new(0., 60., 100., 40.));
                }),
            ])
            .draw(Area::new(0., 0., 100., 100.));
        }

        #[test]
        fn test_configurable_depth_tree() {
            fn build_tree(depth: usize) -> Layout<Area> {
                if depth == 0 {
                    draw(|area| area)
                } else {
                    column(vec![draw(|area| area), build_tree(depth - 1)])
                }
            }

            let mut layout = build_tree(500);
            let leaves = layout.draw(Area::new(0., 0., 300., 300.));
            assert_eq!(leaves.len(), 501);
        }
    }
}
