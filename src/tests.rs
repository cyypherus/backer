#[cfg(test)]
mod tests_module {
    use crate::nodes::*;
    use crate::*;

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
            draw(|area| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 50.0);
            })
            .height(50.0),
            draw(|area| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 50.0);
                assert_eq!(area.y, 50.0);
            })
            .height(50.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_simple_row_layout() {
        row(vec![
            draw(|area| {
                assert_eq!(area.width, 50.0);
                assert_eq!(area.height, 100.0);
            })
            .width(50.0),
            draw(|area| {
                assert_eq!(area.width, 50.0);
                assert_eq!(area.height, 100.0);
                assert_eq!(area.x, 50.0);
            })
            .width(50.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_nested_layout() {
        column(vec![
            row(vec![
                draw(|area| {
                    assert_eq!(area.width, 50.0);
                    assert_eq!(area.height, 25.0);
                })
                .width(50.0),
                draw(|area| {
                    assert_eq!(area.width, 50.0);
                    assert_eq!(area.height, 25.0);
                    assert_eq!(area.x, 50.0);
                })
                .width(50.0),
            ])
            .height(25.0),
            draw(|area| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 75.0);
                assert_eq!(area.y, 25.0);
            })
            .height(75.0),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_padding() {
        draw(|area| {
            assert_eq!(area.x, 10.0);
            assert_eq!(area.y, 10.0);
            assert_eq!(area.width, 80.0);
            assert_eq!(area.height, 80.0);
        })
        .pad(10.0)
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_stack_layout() {
        let values = stack(vec![
            draw(|area| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 100.0);
                1
            }),
            draw(|area| {
                assert_eq!(area.width, 100.0);
                assert_eq!(area.height, 100.0);
                1
            }),
        ])
        .draw(Area::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_dynamic_node() {
        draw(|area| {
            assert_eq!(area.width, 100.0);
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
                draw(|a| {
                    assert_eq!(a, Area::new(50., 0., 50., 20.));
                })
                .height(20.),
            ]),
            draw(|a| {
                assert_eq!(a, Area::new(0., 20., 100., 80.));
            }),
        ])
        .draw(Area::new(0., 0., 100., 100.));
    }

    #[test]
    fn test_static_vs_dynamic_height() {
        column(vec![
            row(vec![
                space().height(0.),
                draw(|a| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ]),
            draw(|a| {
                assert_eq!(a, Area::new(0., 30., 100., 70.));
            }),
        ])
        .draw(Area::new(0., 0., 100., 100.));

        column(vec![
            row(vec![
                space().height(0.),
                draw(|a| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ]),
            draw(|a| {
                assert_eq!(a, Area::new(0., 30., 100., 70.));
            }),
        ])
        .draw(Area::new(0., 0., 100., 100.));
    }

    // #[cfg(test)]
    // mod layout_tests {

    //     use super::*;
    //     #[test]
    //     fn test_seq_align_on_axis() {
    //         row_aligned(
    //             Align::Leading,
    //             vec![
    //                 draw(|a| {
    //                     assert_eq!(a, Area::new(0., 0., 10., 100.));
    //                 })
    //                 .width(10.),
    //                 draw(|a| {
    //                     assert_eq!(a, Area::new(10., 0., 30., 100.));
    //                 })
    //                 .width(30.),
    //             ],
    //         )
    //         .expand()
    //         .draw(Area::new(0., 0., 100., 100.));
    //         row(vec![
    //             draw(|a| {
    //                 assert_eq!(a, Area::new(30., 0., 10., 100.));
    //             })
    //             .width(10.),
    //             draw(|a| {
    //                 assert_eq!(a, Area::new(40., 0., 30., 100.));
    //             })
    //             .width(30.),
    //         ])
    //         .align(Align::CenterX)
    //         .draw(Area::new(0., 0., 100., 100.));
    //         Layout::new({
    //             row_aligned(
    //                 Align::Trailing,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(60., 0., 10., 100.));
    //                     })
    //                     .width(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(70., 0., 30., 100.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_aligned(
    //                 Align::Top,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 0., 100., 10.));
    //                     })
    //                     .height(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 10., 100., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 30., 100., 10.));
    //                 })
    //                 .height(10.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 40., 100., 30.));
    //                 })
    //                 .height(30.),
    //             ])
    //             .align(Align::CenterY)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_aligned(
    //                 Align::Bottom,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 60., 100., 10.));
    //                     })
    //                     .height(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 70., 100., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_seq_align_off_axis() {
    //         Layout::new({
    //             column_aligned(
    //                 Align::Leading,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 0., 10., 50.));
    //                     })
    //                     .width(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 50., 30., 50.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(45., 0., 10., 50.));
    //                 })
    //                 .width(10.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(35., 50., 30., 50.));
    //                 })
    //                 .width(30.),
    //             ])
    //             .align(Align::CenterX)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_aligned(
    //                 Align::Trailing,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(90., 0., 10., 50.));
    //                     })
    //                     .width(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(70., 50., 30., 50.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row_aligned(
    //                 Align::Top,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 0., 50., 10.));
    //                     })
    //                     .height(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(50., 0., 50., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 45., 50., 10.));
    //                 })
    //                 .height(10.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 35., 50., 30.));
    //                 })
    //                 .height(30.),
    //             ])
    //             .align(Align::CenterY)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row_aligned(
    //                 Align::Bottom,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 90., 50., 10.));
    //                     })
    //                     .height(10.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(50., 70., 50., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_seq_align_on_axis_nested_seq() {
    //         Layout::new({
    //             row_aligned(
    //                 Align::Leading,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(0., 0., 10., 100.));
    //                         })
    //                         .width(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(10., 0., 30., 100.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 row(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(30., 0., 10., 100.));
    //                     })
    //                     .width(10.),
    //                 ]),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(40., 0., 30., 100.));
    //                 })
    //                 .width(30.),
    //             ])
    //             .align(Align::CenterX)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row_aligned(
    //                 Align::Trailing,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(60., 0., 10., 100.));
    //                         })
    //                         .width(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(70., 0., 30., 100.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_aligned(
    //                 Align::Top,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(0., 0., 100., 10.));
    //                         })
    //                         .height(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 10., 100., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 row(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 30., 100., 10.));
    //                     })
    //                     .height(10.),
    //                 ]),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 40., 100., 30.));
    //                 })
    //                 .height(30.),
    //             ])
    //             .align(Align::CenterY)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_aligned(
    //                 Align::Bottom,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(0., 60., 100., 10.));
    //                         })
    //                         .height(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 70., 100., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_seq_align_off_axis_nested_seq() {
    //         Layout::new({
    //             column_aligned(
    //                 Align::Leading,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(0., 0., 10., 50.));
    //                         })
    //                         .width(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 50., 30., 50.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 row(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(45., 0., 10., 50.));
    //                     })
    //                     .width(10.),
    //                 ]),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(35., 50., 30., 50.));
    //                 })
    //                 .width(30.),
    //             ])
    //             .align(Align::CenterX)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_aligned(
    //                 Align::Trailing,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(90., 0., 10., 50.));
    //                         })
    //                         .width(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(70., 50., 30., 50.));
    //                     })
    //                     .width(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row_aligned(
    //                 Align::Top,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(0., 0., 50., 10.));
    //                         })
    //                         .height(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(50., 0., 50., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 row(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 45., 50., 10.));
    //                     })
    //                     .height(10.),
    //                 ]),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 35., 50., 30.));
    //                 })
    //                 .height(30.),
    //             ])
    //             .align(Align::CenterY)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row_aligned(
    //                 Align::Bottom,
    //                 vec![
    //                     row(vec![
    //                         draw(|a, _, _| {
    //                             assert_eq!(a, Area::new(0., 90., 50., 10.));
    //                         })
    //                         .height(10.),
    //                     ]),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(50., 70., 50., 30.));
    //                     })
    //                     .height(30.),
    //                 ],
    //             )
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_aspect_ratio() {
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 0., 100., 100.));
    //             })
    //             .aspect_width(1.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(25., 0., 50., 100.));
    //             })
    //             .aspect_width(0.5)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 0., 50., 100.));
    //             })
    //             .aspect_width(0.5)
    //             .align(Align::Leading)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(50., 0., 50., 100.));
    //             })
    //             .aspect_width(0.5)
    //             .align(Align::Trailing)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());

    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 25., 100., 50.));
    //             })
    //             .aspect_height(2.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 0., 100., 50.));
    //             })
    //             .aspect_height(2.)
    //             .align(Align::Top)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 50., 100., 50.));
    //             })
    //             .aspect_height(2.)
    //             .align(Align::Bottom)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_aspect_ratio_in_seq() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 100.));
    //                 })
    //                 .aspect_width(1.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 0., 50., 100.));
    //                 })
    //                 .aspect_width(0.5),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 50., 100.));
    //                 })
    //                 .aspect_width(0.5)
    //                 .align(Align::Leading),
    //             ])
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 0., 50., 100.));
    //                 })
    //                 .aspect_width(0.5)
    //                 .align(Align::Trailing),
    //             ])
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_aspect_ratio_nested() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 200., 50.));
    //                 }),
    //                 row(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 50., 150., 50.));
    //                     }),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(150., 50., 50., 50.));
    //                     })
    //                     .aspect_width(1.),
    //                 ]),
    //             ])
    //         })
    //         .debug_visualize(Area::new(0., 0., 200., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_pad() {
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(10., 10., 80., 80.));
    //             })
    //             .pad(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(10., 0., 80., 100.));
    //             })
    //             .pad_x(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 10., 100., 80.));
    //             })
    //             .pad_y(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(10., 0., 90., 100.));
    //             })
    //             .pad_leading(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 0., 90., 100.));
    //             })
    //             .pad_trailing(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 10., 100., 90.));
    //             })
    //             .pad_top(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 0., 100., 90.));
    //             })
    //             .pad_bottom(10.)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_aspect_ratio_in_pad() {
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(25., 0., 50., 100.));
    //             })
    //             .aspect_width(0.5)
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(30., 10., 40., 80.));
    //                 })
    //                 .aspect_width(0.5)
    //                 .pad(10.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(35., 10., 30., 80.));
    //                 })
    //                 .pad(10.)
    //                 .aspect_width(0.5),
    //             ])
    //         })
    //         .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_aspect_ratio_fit() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 50.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 50., 50., 50.));
    //                 })
    //                 .aspect_width(1.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 0., 50., 50.));
    //                 })
    //                 .aspect_width(1.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 50., 50., 50.));
    //                 })
    //                 .aspect_width(1.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 50., 100.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 25., 50., 50.));
    //                 })
    //                 .aspect_height(1.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 25., 50., 50.));
    //                 })
    //                 .aspect_height(1.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 25., 50., 50.));
    //                 })
    //                 .aspect_height(1.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_space_expansion() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 1., 100.));
    //                 })
    //                 .width(1.),
    //                 space(),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(998., 0., 1., 100.));
    //                 })
    //                 .width(1.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(999., 0., 1., 100.));
    //                 })
    //                 .width(1.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 1000., 100.), &mut (), &mut ());
    //     }
    //     // #[test]
    //     // fn test_explicit_aspect() {
    //     //     Layout::new({
    //     //         column_spaced(
    //     //             10.,
    //     //             vec![
    //     //                 draw(|a, _, _| {
    //     //                     assert_eq!(a, Area::new(45., 0., 10., 20.));
    //     //                 })
    //     //                 .width(10.)
    //     //                 .aspect_width(0.5),
    //     //                 draw(|a, _, _| {
    //     //                     // assert_eq!(a, Area::new(0., 30., 100., 70.));
    //     //                 }),
    //     //             ],
    //     //         )
    //     //     })
    //     //     .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     // }
    //     #[test]
    //     fn test_explicit_with_padding() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(10., 10., 80., 20.));
    //                 })
    //                 .height(20.)
    //                 .pad(10.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 40., 100., 60.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_explicit_in_explicit() {
    //         Layout::new({
    //             draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(40., 0., 20., 100.));
    //             })
    //             .width_range(20.0..)
    //             .pad(0.)
    //             .attach_under(draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(40., 0., 20., 100.));
    //             }))
    //             .width_range(..10.)
    //             .attach_under(draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(45., 0., 10., 100.));
    //             }))
    //         })
    //         .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_compressed_expanded_respects_lower_bound() {
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., -50., 100., 200.));
    //                 })
    //                 .height(200.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., -50., 100., 200.));
    //                 }),
    //             ])
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 stack(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., -50., 100., 200.));
    //                     })
    //                     .height(200.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., -50., 100., 200.));
    //                     }),
    //                 ])
    //                 .expand(),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_compressed_aspect_ratio() {
    //         Layout::<(), ()>::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 25., 50., 50.));
    //                 })
    //                 .aspect_width(1.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 0., 50., 100.));
    //                 })
    //                 .width(50.),
    //             ])
    //             .attach_under(draw(|a, _, _| {
    //                 assert_eq!(a, Area::new(0., 0., 100., 100.));
    //             }))
    //         })
    //         .debug_visualize(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_dynamic_attached() {
    //         Layout::new({
    //             row(vec![
    //                 space(),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 25., 25., 50.));
    //                 })
    //                 .dynamic_height(|h, _, _| h * 2.)
    //                 .attach_under(draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 25., 25., 50.));
    //                 })),
    //                 space(),
    //                 space(),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    // }

    // #[cfg(test)]
    // mod sequence_tests {
    //     use super::*;
    //     #[test]
    //     fn test_column_basic() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 50.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 50., 100., 50.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_column_constrained_1() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 10.));
    //                 })
    //                 .height(10.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 10., 100., 90.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 10.));
    //                 })
    //                 .height(10.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 10., 100., 90.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_column_constrained_2() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 90.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 90., 100., 10.));
    //                 })
    //                 .height(10.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 90.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 90., 100., 10.));
    //                 })
    //                 .height(10.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_row_basic() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 50., 100.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 0., 50., 100.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_row_constrained_1() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 25., 10., 50.));
    //                 })
    //                 .width(10.)
    //                 .height(50.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(10., 0., 90., 100.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::Top),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(10., 40., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(20., 80., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::Bottom),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(30., 0., 70., 100.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_row_constrained_2() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 70., 100.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(70., 0., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::Top),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(80., 40., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(90., 80., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::Bottom),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 70., 100.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(70., 0., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::Top),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(80., 40., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(90., 80., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::Bottom),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_stack_basic() {
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 100.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 100.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }

    //     #[test]
    //     fn test_stack_alignment() {
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::TopLeading),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(45., 0., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::TopCenter),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(90., 0., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::TopTrailing),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(90., 40., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::CenterTrailing),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(90., 80., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::BottomTrailing),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(45., 80., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::BottomCenter),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 80., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::BottomLeading),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 40., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::CenterLeading),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(45., 40., 10., 20.));
    //                 })
    //                 .width(10.)
    //                 .height(20.)
    //                 .align(Align::CenterCenter),
    //             ])
    //             .expand()
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_sequence_spacing() {
    //         Layout::new({
    //             row_spaced(
    //                 10.,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 40., 10., 20.));
    //                     })
    //                     .width(10.)
    //                     .height(20.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(20., 0., 25., 100.));
    //                     }),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(55., 40., 10., 20.));
    //                     })
    //                     .width(10.)
    //                     .height(20.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(75., 0., 25., 100.));
    //                     }),
    //                 ],
    //             )
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //         Layout::new({
    //             column_spaced(
    //                 10.,
    //                 vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 0., 100., 15.));
    //                     }),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(45., 25., 10., 20.));
    //                     })
    //                     .width(10.)
    //                     .height(20.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 55., 100., 15.));
    //                     }),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(45., 80., 10., 20.));
    //                     })
    //                     .width(10.)
    //                     .height(20.),
    //                 ],
    //             )
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    //     #[test]
    //     fn test_row_with_constrained_item() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 30., 100.));
    //                 })
    //                 .width(30.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(30., 0., 70., 100.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }

    //     #[test]
    //     fn test_nested_row_with_constrained_item() {
    //         Layout::new({
    //             row(vec![
    //                 row(vec![
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(0., 0., 20., 100.));
    //                     })
    //                     .width(20.),
    //                     draw(|a, _, _| {
    //                         assert_eq!(a, Area::new(20., 0., 30., 100.));
    //                     }),
    //                 ])
    //                 .width(50.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(50., 0., 50., 100.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }

    //     #[test]
    //     fn test_stack_with_constrained_item() {
    //         Layout::new({
    //             stack(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 100., 100.));
    //                 }),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(25., 25., 50., 50.));
    //                 })
    //                 .width(50.)
    //                 .height(50.),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }

    //     #[test]
    //     fn test_row_with_multiple_constrained_items() {
    //         Layout::new({
    //             row(vec![
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(0., 0., 20., 100.));
    //                 })
    //                 .width(20.),
    //                 draw(|a, _, _| {
    //                     assert_eq!(a, Area::new(20., 0., 30., 100.));
    //                 })
    //                 .width(30.),
    //                 draw(|a, _, _| {
    //                     assert!((a.x - 50.0).abs() < 0.001);
    //                     assert!((a.y - 0.0).abs() < 0.001);
    //                     assert!((a.width - 50.0).abs() < 0.001);
    //                     assert!((a.height - 100.0).abs() < 0.001);
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }

    //     #[test]
    //     fn test_row_with_constrained_height_in_column() {
    //         Layout::new({
    //             column(vec![
    //                 draw(|a, _, _| {
    //                     // Should get 40px height (half of remaining 80px after row takes 20px)
    //                     assert_eq!(a, Area::new(0., 0., 100., 40.));
    //                 }),
    //                 row(vec![
    //                     dynamic(|_, _| {
    //                         draw(|a, _, _| {
    //                             // Row content should be 20px tall
    //                             assert_eq!(a, Area::new(0., 40., 50., 20.));
    //                         })
    //                         .height(20.)
    //                     }),
    //                     dynamic(|_, _| {
    //                         draw(|a, _, _| {
    //                             // Row content should be 20px tall
    //                             assert_eq!(a, Area::new(50., 40., 50., 20.));
    //                         })
    //                         .height(20.)
    //                     }),
    //                 ]),
    //                 draw(|a, _, _| {
    //                     // Should get 40px height (half of remaining 80px after row takes 20px)
    //                     assert_eq!(a, Area::new(0., 60., 100., 40.));
    //                 }),
    //             ])
    //         })
    //         .draw(Area::new(0., 0., 100., 100.), &mut (), &mut ());
    //     }
    // }
}
