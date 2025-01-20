// #[cfg(test)]
// mod tests {
//     use crate::layout::*;
//     use crate::models::*;
//     use crate::nodes::*;
//     use crate::Node;
//     #[test]
//     fn test_scope() {
//         struct A {
//             test: bool,
//             b: B,
//         }
//         struct B {
//             test: bool,
//         }
//         let mut a = A {
//             test: true,
//             b: B { test: true },
//         };
//         fn layout(a: &mut A) -> Node<A> {
//             stack(vec![
//                 if a.test {
//                     draw(|area, a: &mut A| {
//                         assert_eq!(area, Area::new(0., 0., 100., 100.));
//                         a.test = false;
//                     })
//                 } else {
//                     draw(|area, a: &mut A| {
//                         assert_eq!(area, Area::new(0., 0., 100., 100.));
//                         a.test = true;
//                     })
//                 },
//                 draw(|area, a: &mut A| {
//                     Layout::new(|b: &mut B|
//                         if b.test {
//                             draw(|area, b: &mut B| {
//                                 assert_eq!(area, Area::new(0., 0., 100., 100.));
//                                 b.test = false;
//                             })
//                         } else {
//                             draw(|area, b: &mut B| {
//                                 assert_eq!(area, Area::new(0., 0., 100., 100.));
//                                 b.test = true;
//                             })
//                         }
//                     })
//                     .draw(area, &mut a.b);
//                 }),
//             ])
//         }
//         Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
//         assert!(!a.test);
//         assert!(!a.b.test);
//         Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
//         assert!(a.test);
//         assert!(a.b.test);
//     }
//     #[test]
//     fn test_partial_scope_variadic() {
//         struct A;
//         struct C;
//         struct B {
//             c: C,
//         }
//         type State<'a> = (&'a mut A, &'a mut B);
//         type SubState<'a> = (&'a mut A, &'a mut C);
//         fn layout<'a>(_: &mut State) -> Node<State<'a>> {
//             stack(vec![
//                 draw(|area, _state: &mut State| {
//                     assert_eq!(area, Area::new(0., 0., 100., 100.));
//                 }),
//                 draw(|area, state: &mut State| {
//                     Layout::new(|_state: &mut SubState| {
//                         draw(|area, _state: &mut SubState| {
//                             assert_eq!(area, Area::new(0., 0., 100., 100.));
//                         })
//                     })
//                     .draw(area, &mut (&mut state.0, &mut state.1.c));
//                 }),
//             ])
//         }
//         let mut state = (&mut A, &mut B { c: C });
//         Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut state);
//     }
//     #[test]
//     fn test_multiple_scope_paths() {
//         struct C;
//         struct B;
//         struct A {
//             b: B,
//             c: C,
//         }

//         fn path_b(_: &mut A) -> Node<A> {
//             stack(vec![draw(|area, state: &mut A| {
//                 Layout::new(|_state: &mut B| {
//                     draw(|area, _state: &mut B| {
//                         assert_eq!(area, Area::new(0., 0., 100., 100.));
//                     })
//                 })
//                 .draw(area, &mut state.b);
//             })])
//         }
//         fn path_c(_: &mut A) -> Node<A> {
//             stack(vec![draw(|area, state: &mut A| {
//                 Layout::new(|_state: &mut C| {
//                     draw(|area, _state: &mut C| {
//                         assert_eq!(area, Area::new(0., 0., 100., 100.));
//                     })
//                 })
//                 .draw(area, &mut state.c);
//             })])
//         }
//         Layout {
//             tree: stack(vec![path_b(a), path_c(a)]),
//         }
//         .draw(Area::new(0., 0., 100., 100.), &mut A { b: B, c: C });
//     }
//     #[test]
//     fn test_scope_unwrap() {
//         struct B;
//         struct A {
//             b: Option<B>,
//         }
//         fn layout(_a: &mut A) -> Node<A> {
//             stack(vec![draw(|area, state: &mut A| {
//                 if let Some(ref mut b) = state.b {
//                     Layout::new(|_state: &mut B| {
//                         draw(|area, _state: &mut B| {
//                             assert_eq!(area, Area::new(0., 0., 100., 100.));
//                         })
//                     })
//                     .draw(area, b);
//                 }
//             })])
//         }
//         Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: Some(B) });
//     }
//     #[test]
//     fn test_scope_unwrap_ctx() {
//         struct B;
//         struct A {
//             b: Option<B>,
//         }
//         type State<'a> = (&'a mut A, &'a mut B);
//         type SubState<'a> = (&'a mut B, &'a mut B);
//         let mut state = (&mut A { b: Some(B) }, &mut B);
//         fn layout<'a>(_state: &mut State) -> Node<'a, State<'a>> {
//             stack(vec![draw(|area, state: &mut State| {
//                 if let (
//                     A {
//                         b: Some(ref mut scoped_a),
//                     },
//                     ref mut b,
//                 ) = state
//                 {
//                     Layout::new(|_substate: &mut SubState| draw(|_: Area, _: &mut SubState| {}))
//                         .draw(area, &mut (scoped_a, b));
//                 }
//             })])
//         }
//         Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut state);
//     }
// }
