// use backer::{models::*, nodes::*, *};
// use criterion::{Criterion, black_box, criterion_group, criterion_main};

// type TestState = ();
// type TestUIState = ();

// // fn create_simple_column_original() -> Layout<'static, TestState, TestUIState> {
// //     Layout::new(nodes::column(vec![
// //         nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(30.0),
// //         nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(40.0),
// //         nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(25.0),
// //     ]))
// // }

// fn create_simple_column_mvp() -> Layout<TestState, TestUIState> {
//     let mvp_node = column(vec![
//         draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(30.0),
//         draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(40.0),
//         draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(25.0),
//     ]);

//     Layout::new(mvp_node)
// }

// // fn create_complex_nested_original() -> Layout<'static, TestState, TestUIState> {
// //     Layout::new(nodes::column(vec![
// //         nodes::row(vec![
// //             nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                 .width(50.0)
// //                 .height(30.0),
// //             nodes::column(vec![
// //                 nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                     .height(15.0),
// //                 nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                     .height(15.0),
// //             ]),
// //             nodes::stack(vec![
// //                 nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}),
// //                 nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                     .width(20.0)
// //                     .height(20.0),
// //             ]),
// //         ]),
// //         nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(40.0),
// //         nodes::row(vec![
// //             nodes::space(),
// //             nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                 .width(100.0),
// //             nodes::space(),
// //         ]),
// //     ]))
// // }

// fn create_complex_nested_mvp() -> Layout<TestState, TestUIState> {
//     let mvp_node = column(vec![
//         row(vec![
//             draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
//                 .width(50.0)
//                 .height(30.0),
//             column(vec![
//                 draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(15.0),
//                 draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(15.0),
//             ]),
//             stack(vec![
//                 draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}),
//                 draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
//                     .width(20.0)
//                     .height(20.0),
//             ]),
//         ]),
//         draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(40.0),
//         row(vec![
//             space(),
//             draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).width(100.0),
//             space(),
//         ]),
//     ]);

//     Layout::new(mvp_node)
// }

// // fn create_deep_nesting_original(depth: usize) -> Layout<'static, TestState, TestUIState> {
// //     fn create_nested_column(depth: usize) -> backer::Node<'static, TestState, TestUIState> {
// //         if depth == 0 {
// //             nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                 .height(20.0)
// //         } else {
// //             nodes::column(vec![
// //                 create_nested_column(depth - 1),
// //                 create_nested_column(depth - 1),
// //                 create_nested_column(depth - 1),
// //             ])
// //         }
// //     }

// //     Layout::new(create_nested_column(depth))
// // }

// fn create_deep_nesting_mvp(depth: usize) -> Layout<TestState, TestUIState> {
//     fn create_nested_column(depth: usize) -> Node<TestState, TestUIState> {
//         if depth == 0 {
//             draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(20.0)
//         } else {
//             column(vec![
//                 create_nested_column(depth - 1),
//                 create_nested_column(depth - 1),
//                 create_nested_column(depth - 1),
//             ])
//         }
//     }

//     let mvp_node = create_nested_column(depth);
//     Layout::new(mvp_node)
// }

// // fn create_wide_layout_original(width: usize) -> Layout<'static, TestState, TestUIState> {
// //     let elements: Vec<_> = (0..width)
// //         .map(|_| {
// //             nodes::draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {})
// //                 .height(20.0)
// //         })
// //         .collect();

// //     Layout::new(nodes::column(elements))
// // }

// fn create_wide_layout_mvp(width: usize) -> Layout<TestState, TestUIState> {
//     let elements: Vec<_> = (0..width)
//         .map(|_| draw(|_area, _state: &mut TestState, _ui_state: &mut TestUIState| {}).height(20.0))
//         .collect();

//     let mvp_node = column(elements);
//     Layout::new(mvp_node)
// }

// fn bench_simple_column(c: &mut Criterion) {
//     let mut group = c.benchmark_group("simple_column");

//     // group.bench_function("original", |b| {
//     //     b.iter(|| {
//     //         let mut layout = create_simple_column_original();
//     //         let area = Area::new(0.0, 0.0, 100.0, 100.0);
//     //         layout.draw(black_box(area), &mut (), &mut ());
//     //     })
//     // });

//     group.bench_function("mvp", |b| {
//         b.iter(|| {
//             let mut layout = create_simple_column_mvp();
//             let area = Area::new(0.0, 0.0, 100.0, 100.0);
//             layout.draw(black_box(area), &mut (), &mut ());
//         })
//     });

//     group.finish();
// }

// fn bench_complex_nested(c: &mut Criterion) {
//     let mut group = c.benchmark_group("complex_nested");

//     // group.bench_function("original", |b| {
//     //     b.iter(|| {
//     //         let mut layout = create_complex_nested_original();
//     //         let area = Area::new(0.0, 0.0, 400.0, 300.0);
//     //         layout.draw(black_box(area), &mut (), &mut ());
//     //     })
//     // });

//     group.bench_function("mvp", |b| {
//         b.iter(|| {
//             let mut layout = create_complex_nested_mvp();
//             let area = Area::new(0.0, 0.0, 400.0, 300.0);
//             layout.draw(black_box(area), &mut (), &mut ());
//         })
//     });

//     group.finish();
// }

// fn bench_deep_nesting(c: &mut Criterion) {
//     let mut group = c.benchmark_group("deep_nesting");

//     for depth in [3, 5, 7].iter() {
//         // group.bench_with_input(format!("original_depth_{}", depth), depth, |b, &depth| {
//         //     b.iter(|| {
//         //         let mut layout = create_deep_nesting_original(depth);
//         //         let area = Area::new(0.0, 0.0, 200.0, 200.0);
//         //         layout.draw(black_box(area), &mut (), &mut ());
//         //     })
//         // });

//         group.bench_with_input(format!("mvp_depth_{}", depth), depth, |b, &depth| {
//             b.iter(|| {
//                 let mut layout = create_deep_nesting_mvp(depth);
//                 let area = Area::new(0.0, 0.0, 200.0, 200.0);
//                 layout.draw(black_box(area), &mut (), &mut ());
//             })
//         });
//     }

//     group.finish();
// }

// fn bench_wide_layout(c: &mut Criterion) {
//     let mut group = c.benchmark_group("wide_layout");

//     for width in [10, 25, 50].iter() {
//         // group.bench_with_input(format!("original_width_{}", width), width, |b, &width| {
//         //     b.iter(|| {
//         //         let mut layout = create_wide_layout_original(width);
//         //         let area = Area::new(0.0, 0.0, 500.0, 500.0);
//         //         layout.draw(black_box(area), &mut (), &mut ());
//         //     })
//         // });

//         group.bench_with_input(format!("mvp_width_{}", width), width, |b, &width| {
//             b.iter(|| {
//                 let mut layout = create_wide_layout_mvp(width);
//                 let area = Area::new(0.0, 0.0, 500.0, 500.0);
//                 layout.draw(black_box(area), &mut (), &mut ());
//             })
//         });
//     }

//     group.finish();
// }

// fn bench_reuse_scenarios(c: &mut Criterion) {
//     let mut group = c.benchmark_group("reuse_scenarios");

//     group.bench_function("mvp_multiple_frames", |b| {
//         let mut layout = create_complex_nested_mvp();
//         let area = Area::new(0.0, 0.0, 400.0, 300.0);

//         b.iter(|| {
//             layout.draw(black_box(area), &mut (), &mut ());
//         })
//     });

//     // group.bench_function("original_multiple_frames", |b| {
//     //     let mut layout = create_complex_nested_original();
//     //     let area = Area::new(0.0, 0.0, 400.0, 300.0);

//     //     b.iter(|| {
//     //         layout.draw(black_box(area), &mut (), &mut ());
//     //     })
//     // });

//     group.finish();
// }

// criterion_group!(
//     benches,
//     bench_simple_column,
//     bench_complex_nested,
//     bench_deep_nesting,
//     bench_wide_layout,
//     bench_reuse_scenarios
// );
// criterion_main!(benches);
fn main() {
    // Add your main function logic here
}
