#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;

    #[test]
    fn test_scope() {
        #[derive(Debug)]
        struct A {
            test: bool,
            b: B,
        }
        #[derive(Debug)]
        struct B {
            test: bool,
        }
        let mut a = A {
            test: true,
            b: B { test: true },
        };
        let layout = dynamic(|a: &mut A| {
            stack(vec![
                {
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
                    }
                },
                scope_ref(
                    |a: &mut A| &mut a.b,
                    |b| {
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
                    },
                ),
            ])
        });
        let mut layout = Layout::new(layout);
        layout.draw(Area::new(0., 0., 100., 100.), &mut a);
        assert!(!a.test);
        assert!(!a.b.test);
        layout.draw(Area::new(0., 0., 100., 100.), &mut a);
        assert!(a.test);
        assert!(a.b.test);
    }

    #[test]
    fn test_multiple_scope_paths() {
        struct C;
        struct B;
        struct A {
            b: B,
            c: C,
        }
        Layout::new(stack(vec![
            scope_ref(
                |a: &mut A| &mut a.b,
                |_| {
                    draw(|area, _state: &mut B| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                    })
                },
            ),
            scope_ref(
                |a: &mut A| &mut a.c,
                |_| {
                    draw(|area, _state: &mut C| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                    })
                },
            ),
        ]))
        .draw(Area::new(0., 0., 100., 100.), &mut A { b: B, c: C });
    }

    #[test]
    fn test_scope_unwrap() {
        struct B {
            test: bool,
        }
        struct A {
            b: Option<B>,
        }
        let layout = dynamic(|_: &mut A| {
            stack(vec![scope_option(
                |a: &mut A| a.b.take(),
                |a: &mut A, b: B| a.b = Some(b),
                |_| {
                    draw(|area, state: &mut B| {
                        state.test = !state.test;
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                    })
                },
            )])
        });
        let mut state = A {
            b: Some(B { test: false }),
        };
        let mut layout = Layout::new(layout);
        layout.draw(Area::new(0., 0., 100., 100.), &mut state);
        assert!(state.b.as_ref().unwrap().test);
        layout.draw(Area::new(0., 0., 100., 100.), &mut state);
        assert!(!state.b.as_ref().unwrap().test);
    }

    #[test]
    fn test_scope_inv() {
        struct B;
        struct A {
            b: B,
        }
        type One<'a> = (&'a mut A, &'a mut A);
        type Two<'a> = (&'a mut B, &'a mut A);

        let mut one = A { b: B };
        let mut oneone = A { b: B };
        Layout::new(stack(vec![scope_owned(
            |a: &mut One| (&mut a.0.b, a.1),
            |a: &mut One, b: Two| {},
            |_: &mut Two| {
                draw(|area, _state: &mut Two| {
                    assert_eq!(area, Area::new(0., 0., 100., 100.));
                })
            },
        )]))
        .draw(Area::new(0., 0., 100., 100.), &mut (&mut one, &mut oneone));
    }
}
