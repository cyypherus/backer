#[cfg(test)]
mod tests {

    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    use crate::traits::Scopable;
    use crate::traits::ScopableOption;
    use crate::Node;
    use crate::NodeWith;
    #[test]
    fn test_seq_align_on_axis() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(40., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(60., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 30., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 60., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 70., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_off_axis() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(35., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 45., 50., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 35., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 50., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 70., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_on_axis_nested_seq() {
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(40., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(60., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 30., 100.));
                })
                .width(30.),
            ])
            .align(Align::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 30., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 60., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 70., 100., 30.));
                })
                .height(30.),
            ])
            .align(Align::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_off_axis_nested_seq() {
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(35., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::CenterX)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(90., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 50., 30., 50.));
                })
                .width(30.),
            ])
            .align(Align::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 45., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 35., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::CenterY)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 70., 50., 30.));
                })
                .height(30.),
            ])
            .align(Align::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio() {
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 100.));
            })
            .aspect(1.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 50., 100.));
            })
            .aspect(0.5)
            .align(Align::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(50., 0., 50., 100.));
            })
            .aspect(0.5)
            .align(Align::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());

        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 25., 100., 50.));
            })
            .aspect(2.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 50.));
            })
            .aspect(2.)
            .align(Align::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 50., 100., 50.));
            })
            .aspect(2.)
            .align(Align::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio_in_seq() {
        Layout::new(|()| {
            row(vec![draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 100.));
            })
            .aspect(1.)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            stack(vec![draw(|a, _| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![draw(|a, _| {
                assert_eq!(a, Area::new(0., -50., 100., 200.));
            })
            .aspect(0.5)
            .align(Align::Leading)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            stack(vec![draw(|a, _| {
                assert_eq!(a, Area::new(50., 0., 50., 100.));
            })
            .aspect(0.5)
            .align(Align::Trailing)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_pad() {
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(10., 10., 80., 80.));
            })
            .pad(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(10., 0., 80., 100.));
            })
            .pad_x(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 10., 100., 80.));
            })
            .pad_y(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(10., 0., 90., 100.));
            })
            .pad_leading(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 90., 100.));
            })
            .pad_trailing(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 10., 100., 90.));
            })
            .pad_top(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 90.));
            })
            .pad_bottom(10.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio_in_pad() {
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            stack(vec![draw(|a, _| {
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
        Layout::new(|()| {
            stack(vec![draw(|a, _| {
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
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 1., 100.));
                })
                .width(1.),
                space(),
                draw(|a, _| {
                    assert_eq!(a, Area::new(998., 0., 1., 100.));
                })
                .width(1.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(999., 0., 1., 100.));
                })
                .width(1.),
            ])
        })
        .draw(Area::new(0., 0., 1000., 100.), &mut ());
    }
    #[test]
    fn test_explicit_aspect() {
        Layout::new(|()| {
            column_spaced(
                10.,
                vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(45., 0., 10., 20.));
                    })
                    .width(10.)
                    .aspect(0.5),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 30., 100., 70.));
                    }),
                ],
            )
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_explicit_with_padding() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 10., 80., 20.));
                })
                .height(20.)
                .pad(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 100., 60.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_scope() {
        struct A {
            test: bool,
            b: B,
        }
        struct B {
            test: bool,
        }
        let mut a = A {
            test: true,
            b: B { test: true },
        };
        impl Scopable<B> for A {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut B) -> Result,
            {
                f(&mut self.b)
            }
        }
        fn layout(a: &mut A) -> NodeWith<A, ()> {
            stack(vec![
                if a.test {
                    draw(|area, a: &mut A| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                        a.test = false;
                    })
                } else {
                    draw(|area, a: &mut A| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                        a.test = true;
                    })
                },
                scope(|b: &mut B| {
                    if b.test {
                        draw(|area, b: &mut B| {
                            assert_eq!(area, Area::new(0., 0., 100., 100.));
                            b.test = false;
                        })
                    } else {
                        draw(|area, b: &mut B| {
                            assert_eq!(area, Area::new(0., 0., 100., 100.));
                            b.test = true;
                        })
                    }
                }),
            ])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
        assert!(!a.test);
        assert!(!a.b.test);
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
        assert!(a.test);
        assert!(a.b.test);
    }
    #[test]
    fn test_partial_scope_variadic() {
        struct A;
        struct C;
        struct B {
            c: C,
        }

        impl Scopable<A> for A {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut A) -> Result,
            {
                f(self)
            }
        }

        impl Scopable<C> for B {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut C) -> Result,
            {
                f(&mut self.c)
            }
        }

        fn layout(_: &mut A, _: &mut B) -> NodeWith<A, B> {
            stack(vec![
                draw_with(|area, _, _| {
                    assert_eq!(area, Area::new(0., 0., 100., 100.));
                }),
                scope_with(|_, _| {
                    draw_with(|area, _a: &mut A, _c: &mut C| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                    })
                }),
            ])
        }
        Layout::new_with(layout).draw_with(Area::new(0., 0., 100., 100.), &mut A, &mut B { c: C });
    }
    #[test]
    fn test_multiple_scope_paths() {
        struct C;
        struct B;
        struct A {
            b: B,
            c: C,
        }
        fn layout(a: &mut A) -> Node<A> {
            stack(vec![path_b(a), path_c(a)])
        }
        fn path_b(_: &mut A) -> Node<A> {
            impl Scopable<B> for A {
                fn scope<F, Result>(&mut self, f: F) -> Result
                where
                    F: FnOnce(&mut B) -> Result,
                {
                    f(&mut self.b)
                }
            }
            stack(vec![scope(|_b: &mut B| space())])
        }
        fn path_c(_: &mut A) -> Node<A> {
            impl Scopable<C> for A {
                fn scope<F, Result>(&mut self, f: F) -> Result
                where
                    F: FnOnce(&mut C) -> Result,
                {
                    f(&mut self.c)
                }
            }
            stack(vec![scope(|_c: &mut C| space())])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: B, c: C });
    }
    #[test]
    fn test_scope_unwrap() {
        struct B;
        struct A {
            b: Option<B>,
        }
        impl ScopableOption<B> for A {
            fn scope_option<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(Option<&mut B>) -> Result,
            {
                f(self.b.as_mut())
            }
        }
        fn layout(_a: &mut A) -> Node<A> {
            stack(vec![scope(|_b: &mut B| space())])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: Some(B) });
    }
    #[test]
    fn test_scope_unwrap_ctx() {
        struct B;
        struct A {
            b: Option<B>,
        }
        impl ScopableOption<B> for A {
            fn scope_option<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(Option<&mut B>) -> Result,
            {
                f(self.b.as_mut())
            }
        }
        impl Scopable<B> for B {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut B) -> Result,
            {
                f(self)
            }
        }
        fn layout(_b: &mut B, _a: &mut A) -> NodeWith<B, A> {
            stack(vec![scope_with(|_b: &mut B, _b_1: &mut B| space())])
        }
        Layout::new_with(layout).draw_with(
            Area::new(0., 0., 100., 100.),
            &mut B,
            &mut A { b: Some(B) },
        );
    }
}
