#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    use crate::Node;

    // #[test]
    // fn test_scope() {
    //     #[derive(Debug)]
    //     struct A {
    //         test: bool,
    //         b: B,
    //     }
    //     #[derive(Debug)]
    //     struct B {
    //         test: bool,
    //     }
    //     let mut a = A {
    //         test: true,
    //         b: B { test: true },
    //     };
    //     let layout = dynamic(|a: &mut A, _| {
    //         stack(vec![
    //             {
    //                 if a.test {
    //                     draw(|area, a: &mut A, _| {
    //                         assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                         a.test = false;
    //                     })
    //                 } else {
    //                     draw(|area, a: &mut A, _| {
    //                         assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                         a.test = true;
    //                     })
    //                 }
    //             },
    //             scope(
    //                 |a: &mut A| &mut a.b,
    //                 |b| b,
    //                 dynamic(|b: &mut B, _| {
    //                     if b.test {
    //                         draw(|area, b: &mut B, _| {
    //                             assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                             b.test = false;
    //                         })
    //                     } else {
    //                         draw(|area, b: &mut B, _| {
    //                             assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                             b.test = true;
    //                         })
    //                     }
    //                 }),
    //             ),
    //         ])
    //     });
    //     let mut layout = Layout::new(layout);
    //     layout.draw(Area::new(0., 0., 100., 100.), &mut a, &mut ());
    //     assert!(!a.test);
    //     assert!(!a.b.test);
    //     layout.draw(Area::new(0., 0., 100., 100.), &mut a, &mut ());
    //     assert!(a.test);
    //     assert!(a.b.test);
    // }

    // #[test]
    // fn test_multiple_scope_paths() {
    //     struct C;
    //     struct B;
    //     struct A {
    //         b: B,
    //         c: C,
    //     }
    //     Layout::new(stack(vec![
    //         scope(
    //             |a: &mut A| &mut a.b,
    //             |b| b,
    //             draw(|area, _state: &mut B, _| {
    //                 assert_eq!(area, Area::new(0., 0., 100., 100.));
    //             }),
    //         ),
    //         scope(
    //             |a: &mut A| &mut a.c,
    //             |b| b,
    //             draw(|area, _state: &mut C, _| {
    //                 assert_eq!(area, Area::new(0., 0., 100., 100.));
    //             }),
    //         ),
    //     ]))
    //     .draw(
    //         Area::new(0., 0., 100., 100.),
    //         &mut A { b: B, c: C },
    //         &mut (),
    //     );
    // }

    // #[test]
    // fn test_scope_unwrap() {
    //     struct B {
    //         test: bool,
    //     }
    //     struct A {
    //         b: Option<B>,
    //     }
    //     let layout = dynamic(|a: &mut A, _| {
    //         stack(vec![
    //             //>
    //             scope_unwrap(
    //                 |a: &mut A| &mut a.b,
    //                 |b| &mut Some(b),
    //                 draw(|_, b: &mut B, _| b.test = !b.test),
    //             ),
    //         ])
    //     });
    //     let mut state = A {
    //         b: Some(B { test: false }),
    //     };
    //     let mut layout = Layout::new(layout);
    //     layout.draw(Area::new(0., 0., 100., 100.), &mut state, &mut ());
    //     assert!(state.b.as_ref().unwrap().test);
    //     layout.draw(Area::new(0., 0., 100., 100.), &mut state, &mut ());
    //     assert!(!state.b.as_ref().unwrap().test);
    // }

    // No can do buddy
    // #[test]
    // fn test_scope_inv() {
    //     struct B;
    //     struct A {
    //         b: B,
    //     }
    //     type One<'a> = (&'a mut A, &'a mut A);
    //     type Two<'a> = (&'a mut B, &'a mut A);

    //     let mut one = A { b: B };
    //     let mut oneone = A { b: B };
    //     Layout::new(stack(vec![
    //         //>
    //         scope_owned(
    //             |a: &mut One| {
    //                 //>
    //                 (&mut a.0.b, a.1.borrow_mut())
    //             },
    //             space(),
    //         ),
    //     ]))
    //     .draw(Area::new(0., 0., 100., 100.), &mut (&mut one, &mut oneone));
    // }

    #[test]
    fn test_lens() {
        struct B {
            c: bool,
        }
        struct A {
            b: B,
        }

        trait CView {
            fn c(&mut self) -> &mut bool;
        }

        impl CView for B {
            fn c(&mut self) -> &mut bool {
                &mut self.c
            }
        }

        fn c_view<'t, T>() -> Node<'t, T, Lens<T, bool>> {
            draw(|_, s| {
                let _view: &mut bool = s;
            })
        }
        fn b_view<'t, T: CView + 't>() -> Node<'t, T, Lens<T, B>> {
            row(vec![
                scope(|t: &mut T| t.c(), c_view()),
                draw(|_, s| {
                    let _view: &mut B = s;
                }),
            ])
        }
        fn a_view() -> Node<'static, A, fn(&mut A) -> &mut A> {
            row(vec![
                draw(|_, a: &mut A| {}),
                scope(|a: &mut A| &mut a.b, b_view()),
            ])
        }

        let mut state = A { b: B { c: true } };
        Layout::new(a_view()).draw(Area::new(0., 0., 100., 100.), &mut state, noop_lens);
    }
}
