#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;

    #[test]
    fn test_seq_align_on_axis() {
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(10., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align_contents(Align::Leading)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(30., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(40., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(60., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(70., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align_contents(Align::Trailing)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 10., 100., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Top)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 30., 100., 10.));
                })
                .height(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 40., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 60., 100., 10.));
                })
                .height(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 70., 100., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Bottom)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_off_axis() {
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align_contents(Align::Leading)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(45., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(35., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(90., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(70., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align_contents(Align::Trailing)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 50., 10.));
                })
                .height(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Top)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 45., 50., 10.));
                })
                .height(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(50., 35., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 90., 50., 10.));
                })
                .height(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(50., 70., 50., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Bottom)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_on_axis_nested_seq() {
        Layout::new({
            row(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(10., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align_contents(Align::Leading)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(30., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(40., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(60., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(70., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align_contents(Align::Trailing)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 10., 100., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Top)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 30., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 40., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 60., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 70., 100., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Bottom)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_off_axis_nested_seq() {
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align_contents(Align::Leading)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(45., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(35., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(90., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(70., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align_contents(Align::Trailing)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Top)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 45., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(50., 35., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            row(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 90., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(50., 70., 50., 30.));
                })
                .height(30.),
            ])
            .align_contents(Align::Bottom)
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio() {
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 0., 100., 100.));
            })
            .aspect(1.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 0., 50., 100.));
            })
            .aspect(0.5)
            .align(Align::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(50., 0., 50., 100.));
            })
            .aspect(0.5)
            .align(Align::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());

        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 25., 100., 50.));
            })
            .aspect(2.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 0., 100., 50.));
            })
            .aspect(2.)
            .align(Align::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 50., 100., 50.));
            })
            .aspect(2.)
            .align(Align::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio_in_seq() {
        Layout::new({
            row(vec![draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 0., 100., 100.));
            })
            .aspect(1.)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            stack(vec![draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., -50., 100., 200.));
            })
            .aspect(0.5)
            .align(Align::Leading)])
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            stack(vec![draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(50., 0., 50., 100.));
            })
            .aspect(0.5)
            .align(Align::Trailing)])
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_pad() {
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(10., 10., 80., 80.));
            })
            .pad(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(10., 0., 80., 100.));
            })
            .pad_x(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 10., 100., 80.));
            })
            .pad_y(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(10., 0., 90., 100.));
            })
            .pad_leading(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 0., 90., 100.));
            })
            .pad_trailing(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 10., 100., 90.));
            })
            .pad_top(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(0., 0., 100., 90.));
            })
            .pad_bottom(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio_in_pad() {
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            stack(vec![draw(|a, _: &mut ()| {
                // 0.5 aspect ratio
                // padded size
                // 10., 10., 80., 80.
                // constrain aspect, width = 0.5 x height
                // item is then centered
                // 30., 10, 40., 80.
                assert_eq!(a, Area::new(30., 10., 40., 80.));
            })
            .aspect(0.5)
            .pad(10.)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            stack(vec![draw(|a, _: &mut ()| {
                // 0.5 aspect ratio
                // aspect constrained size
                // 25., 0., 50., 100.
                // add padding of 10. on every edge to the aspect constrained size
                // 35., 10., 30., 80.
                assert_eq!(a, Area::new(35., 10., 30., 80.));
            })
            .pad(10.)
            .aspect(0.5)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_space_expansion() {
        // The unconstrained space node should expand an unlimited amount
        Layout::new({
            row(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 1., 100.));
                })
                .width(1.),
                space(),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(998., 0., 1., 100.));
                })
                .width(1.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(999., 0., 1., 100.));
                })
                .width(1.),
            ])
        })
        .draw(Area::new(0., 0., 1000., 100.), &mut ());
    }
    #[test]
    fn test_explicit_aspect() {
        Layout::new({
            column_spaced(
                10.,
                vec![
                    draw(|a, _: &mut ()| {
                        assert_eq!(a, Area::new(45., 0., 10., 20.));
                    })
                    .width(10.)
                    .aspect(0.5),
                    draw(|a, _: &mut ()| {
                        assert_eq!(a, Area::new(0., 30., 100., 70.));
                    }),
                ],
            )
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_explicit_with_padding() {
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(10., 10., 80., 20.));
                })
                .height(20.)
                .pad(10.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 40., 100., 60.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_explicit_in_explicit() {
        Layout::new({
            draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(40., 0., 20., 100.));
            })
            .width_range(20.0..)
            .pad(0.)
            .attach_under(draw(|a, _: &mut ()| {
                assert_eq!(a, Area::new(45., 0., 10., 100.));
            }))
            .width_range(..10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_compressed_expanded_respects_lower_bound() {
        Layout::new({
            stack(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., -50., 100., 200.));
                })
                .height(200.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., -50., 100., 200.));
                }),
            ])
            .expand()
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![stack(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., -50., 100., 200.));
                })
                .height(200.),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., -50., 100., 200.));
                }),
            ])
            .expand()])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
}
