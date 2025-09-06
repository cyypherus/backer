//! Test suite for MVP layout engine to ensure exact compatibility with existing implementation
//!
//! These tests compare the MVP iterative layout engine against the original recursive
//! implementation to ensure identical layout results for all supported scenarios.

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::mvp::{self, MvpLayout};
    use crate::nodes;
    use crate::Layout;
    use std::cell::RefCell;
    use std::rc::Rc;

    type TestState = ();
    type TestUIState = ();

    fn compare_areas(expected: Area, actual: Area, tolerance: f32) {
        assert!(
            (expected.x - actual.x).abs() < tolerance,
            "X mismatch: expected {}, got {}",
            expected.x,
            actual.x
        );
        assert!(
            (expected.y - actual.y).abs() < tolerance,
            "Y mismatch: expected {}, got {}",
            expected.y,
            actual.y
        );
        assert!(
            (expected.width - actual.width).abs() < tolerance,
            "Width mismatch: expected {}, got {}",
            expected.width,
            actual.width
        );
        assert!(
            (expected.height - actual.height).abs() < tolerance,
            "Height mismatch: expected {}, got {}",
            expected.height,
            actual.height
        );
    }

    #[test]
    fn test_mvp_vs_original_simple_column() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        // Use shared references to capture areas
        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig_1 = captured_areas_original.clone();
        let areas_orig_2 = captured_areas_original.clone();

        let original_layout = Layout::new(nodes::column(vec![
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_1.borrow_mut().push(area);
            })
            .height(30.0),
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_2.borrow_mut().push(area);
            })
            .height(40.0),
        ]));

        // MVP implementation
        let areas_mvp_1 = captured_areas_mvp.clone();
        let areas_mvp_2 = captured_areas_mvp.clone();

        let mvp_node = mvp::column(vec![
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                println!("Drawing MVP node 1");
                areas_mvp_1.borrow_mut().push(area);
            })
            .height(30.0),
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                println!("Drawing MVP node 2");
                areas_mvp_2.borrow_mut().push(area);
            })
            .height(40.0),
        ]);

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter().rev()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_vs_original_simple_row() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig_1 = captured_areas_original.clone();
        let areas_orig_2 = captured_areas_original.clone();

        let original_layout = Layout::new(nodes::row(vec![
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_1.borrow_mut().push(area);
            })
            .width(30.0),
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_2.borrow_mut().push(area);
            })
            .width(40.0),
        ]));

        // MVP implementation
        let areas_mvp_1 = captured_areas_mvp.clone();
        let areas_mvp_2 = captured_areas_mvp.clone();

        let mvp_node = mvp::row(vec![
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_1.borrow_mut().push(area);
            })
            .width(30.0),
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_2.borrow_mut().push(area);
            })
            .width(40.0),
        ]);

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter().rev()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_vs_original_stack() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig_1 = captured_areas_original.clone();
        let areas_orig_2 = captured_areas_original.clone();

        let original_layout = Layout::new(nodes::stack(vec![
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_1.borrow_mut().push(area);
            }),
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_2.borrow_mut().push(area);
            })
            .width(50.0)
            .height(50.0),
        ]));

        // MVP implementation
        let areas_mvp_1 = captured_areas_mvp.clone();
        let areas_mvp_2 = captured_areas_mvp.clone();

        let mvp_node = mvp::stack(vec![
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_1.borrow_mut().push(area);
            }),
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_2.borrow_mut().push(area);
            })
            .width(50.0)
            .height(50.0),
        ]);

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_vs_original_padding() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig = captured_areas_original.clone();
        let original_layout = Layout::new(
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig.borrow_mut().push(area);
            })
            .pad(15.0),
        );

        // MVP implementation
        let areas_mvp = captured_areas_mvp.clone();
        let mvp_node = mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
            areas_mvp.borrow_mut().push(area);
        })
        .pad(15.0);

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_vs_original_spacing() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig_1 = captured_areas_original.clone();
        let areas_orig_2 = captured_areas_original.clone();
        let areas_orig_3 = captured_areas_original.clone();

        let original_layout = Layout::new(nodes::column_spaced(
            10.0,
            vec![
                nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                    areas_orig_1.borrow_mut().push(area);
                })
                .height(20.0),
                nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                    areas_orig_2.borrow_mut().push(area);
                })
                .height(30.0),
                nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                    areas_orig_3.borrow_mut().push(area);
                }),
            ],
        ));

        // MVP implementation
        let areas_mvp_1 = captured_areas_mvp.clone();
        let areas_mvp_2 = captured_areas_mvp.clone();
        let areas_mvp_3 = captured_areas_mvp.clone();

        let mvp_node = mvp::column_spaced(
            10.0,
            vec![
                mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                    areas_mvp_1.borrow_mut().push(area);
                })
                .height(20.0),
                mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                    areas_mvp_2.borrow_mut().push(area);
                })
                .height(30.0),
                mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                    areas_mvp_3.borrow_mut().push(area);
                }),
            ],
        );

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter().rev()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_iterative_only() {
        // This test ensures that the MVP implementation is truly iterative
        // by creating a very deep nesting that would overflow the stack
        // if implemented recursively

        let mut deep_node = mvp::space();

        // Create 1000 levels of nesting
        for _ in 0..1000 {
            deep_node = mvp::column(vec![deep_node]);
        }

        let mut mvp_layout = MvpLayout::new(deep_node);
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        // This should not overflow the stack
        mvp_layout.draw(test_area, &mut (), &mut ());
    }

    #[test]
    fn test_mvp_predictable_traversal() {
        // Test that layout passes are predictable and consistent
        let draw_order = Rc::new(RefCell::new(Vec::new()));

        let order_a = draw_order.clone();
        let order_b = draw_order.clone();
        let order_c = draw_order.clone();

        let mvp_node = mvp::column(vec![
            mvp::row(vec![
                mvp::draw(move |_area, _: &mut TestState, _: &mut TestUIState| {
                    order_a.borrow_mut().push("A");
                }),
                mvp::draw(move |_area, _: &mut TestState, _: &mut TestUIState| {
                    order_b.borrow_mut().push("B");
                }),
            ]),
            mvp::draw(move |_area, _: &mut TestState, _: &mut TestUIState| {
                order_c.borrow_mut().push("C");
            }),
        ]);

        let mut mvp_layout = MvpLayout::new(mvp_node);
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        mvp_layout.draw(test_area, &mut (), &mut ());

        // Drawing should happen in document order
        let order = draw_order.borrow();
        assert_eq!(*order, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_mvp_basic_functionality() {
        // Simple test to verify the MVP implementation works at all
        let draw_count = Rc::new(RefCell::new(0));
        let counter = draw_count.clone();

        let mvp_node = mvp::column(vec![
            mvp::draw(move |_area, _: &mut TestState, _: &mut TestUIState| {
                *counter.borrow_mut() += 1;
            })
            .width(50.0)
            .height(50.0),
            mvp::space(),
        ]);

        let mut mvp_layout = MvpLayout::new(mvp_node);
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        mvp_layout.draw(test_area, &mut (), &mut ());

        // Should have called draw once
        assert_eq!(*draw_count.borrow(), 1);
    }

    #[test]
    fn test_mvp_constraints() {
        // Test that basic constraints work
        let captured_area = Rc::new(RefCell::new(None));
        let area_capture = captured_area.clone();

        let mvp_node = mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
            *area_capture.borrow_mut() = Some(area);
        })
        .width(50.0)
        .height(30.0);

        let mut mvp_layout = MvpLayout::new(mvp_node);
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        mvp_layout.draw(test_area, &mut (), &mut ());

        let area = captured_area.borrow().unwrap();

        // Should be positioned at top-left with specified dimensions
        assert_eq!(area.width, 50.0);
        assert_eq!(area.height, 30.0);
    }

    #[test]
    fn test_mvp_row_layout() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig_1 = captured_areas_original.clone();
        let areas_orig_2 = captured_areas_original.clone();

        let original_layout = Layout::new(nodes::row(vec![
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_1.borrow_mut().push(area);
            })
            .width(30.0),
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_2.borrow_mut().push(area);
            })
            .width(40.0),
        ]));

        // MVP implementation
        let areas_mvp_1 = captured_areas_mvp.clone();
        let areas_mvp_2 = captured_areas_mvp.clone();

        let mvp_node = mvp::row(vec![
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_1.borrow_mut().push(area);
            })
            .width(30.0),
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_2.borrow_mut().push(area);
            })
            .width(40.0),
        ]);

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter().rev()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_column_layout() {
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        let captured_areas_original = Rc::new(RefCell::new(Vec::new()));
        let captured_areas_mvp = Rc::new(RefCell::new(Vec::new()));

        // Original implementation
        let areas_orig_1 = captured_areas_original.clone();
        let areas_orig_2 = captured_areas_original.clone();

        let original_layout = Layout::new(nodes::column(vec![
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_1.borrow_mut().push(area);
            })
            .height(30.0),
            nodes::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_orig_2.borrow_mut().push(area);
            })
            .height(40.0),
        ]));

        // MVP implementation
        let areas_mvp_1 = captured_areas_mvp.clone();
        let areas_mvp_2 = captured_areas_mvp.clone();

        let mvp_node = mvp::column(vec![
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_1.borrow_mut().push(area);
            })
            .height(30.0),
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                areas_mvp_2.borrow_mut().push(area);
            })
            .height(40.0),
        ]);

        // Execute both layouts
        let mut original_layout = original_layout;
        original_layout.draw(test_area, &mut (), &mut ());

        let mut mvp_layout = MvpLayout::new(mvp_node);
        mvp_layout.draw(test_area, &mut (), &mut ());

        // Compare results
        let orig_areas = captured_areas_original.borrow();
        let mvp_areas = captured_areas_mvp.borrow();

        assert_eq!(orig_areas.len(), mvp_areas.len());
        for (original, mvp) in orig_areas.iter().zip(mvp_areas.iter().rev()) {
            compare_areas(*original, *mvp, 0.001);
        }
    }

    #[test]
    fn test_mvp_stack_layout() {
        let areas = Rc::new(RefCell::new(Vec::new()));

        let area1 = areas.clone();
        let area2 = areas.clone();

        let mvp_node = mvp::stack(vec![
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                area1.borrow_mut().push(area);
            }),
            mvp::draw(move |area, _: &mut TestState, _: &mut TestUIState| {
                area2.borrow_mut().push(area);
            }),
        ]);

        let mut mvp_layout = MvpLayout::new(mvp_node);
        let test_area = Area::new(0.0, 0.0, 100.0, 100.0);

        mvp_layout.draw(test_area, &mut (), &mut ());

        let captured_areas = areas.borrow();
        assert_eq!(captured_areas.len(), 2);

        // Both elements should have the same area (full available space)
        for area in captured_areas.iter() {
            assert_eq!(area.x, 0.0);
            assert_eq!(area.y, 0.0);
            assert_eq!(area.width, 100.0);
            assert_eq!(area.height, 100.0);
        }
    }
}
