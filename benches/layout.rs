use backer::{Area, Layout, nodes::*};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn draw_noop() -> Layout<'static, Area> {
    draw(|area: Area, _: &mut ()| vec![area])
}

fn create_simple_column_layout() -> Layout<'static, Area> {
    column(vec![
        draw_noop().height(30.0),
        draw_noop().height(40.0),
        draw_noop().height(25.0),
    ])
}

fn create_complex_nested_layout() -> Layout<'static, Area> {
    column(vec![
        row(vec![
            draw_noop().width(50.0).height(30.0),
            column(vec![draw_noop().height(15.0), draw_noop().height(15.0)]),
            stack(vec![draw_noop(), draw_noop().width(20.0).height(20.0)]),
        ]),
        draw_noop().height(40.0),
        row(vec![space(), draw_noop().width(100.0), space()]),
    ])
}

fn create_nested_column(depth: usize) -> Layout<'static, Area> {
    if depth == 0 {
        draw_noop().height(20.0)
    } else {
        column(vec![create_nested_column(depth - 1)])
    }
}

fn create_deep_nesting_mvp(depth: usize) -> Layout<'static, Area> {
    create_nested_column(depth)
}

fn create_wide_layout_mvp(width: usize) -> Layout<'static, Area> {
    let elements: Vec<_> = (0..width).map(|_| draw_noop().height(20.0)).collect();
    column(elements)
}

fn bench_simple_column(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_column");

    group.bench_function("mvp", |b| {
        b.iter(|| {
            let mut layout = create_simple_column_layout();
            let area = Area::new(0.0, 0.0, 100.0, 100.0);
            black_box(&mut layout).draw(black_box(area), &mut ());
        })
    });

    group.finish();
}

fn bench_complex_nested(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_nested");

    group.bench_function("mvp", |b| {
        b.iter(|| {
            let mut layout = create_complex_nested_layout();
            let area = Area::new(0.0, 0.0, 400.0, 300.0);
            black_box(&mut layout).draw(black_box(area), &mut ());
        })
    });

    group.finish();
}

fn bench_deep_nesting(c: &mut Criterion) {
    let mut group = c.benchmark_group("deep_nesting");

    for depth in [10, 50, 100] {
        group.bench_with_input(format!("mvp_depth_{}", depth), &depth, |b, &depth| {
            b.iter(|| {
                let mut layout = create_deep_nesting_mvp(depth);
                let area = Area::new(0.0, 0.0, 200.0, 200.0);
                black_box(&mut layout).draw(black_box(area), &mut ());
            })
        });
    }

    group.finish();
}

fn bench_wide_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide_layout");

    for width in [10, 50, 100] {
        group.bench_with_input(format!("mvp_width_{}", width), &width, |b, &width| {
            b.iter(|| {
                let mut layout = create_wide_layout_mvp(width);
                let area = Area::new(0.0, 0.0, 300.0, 300.0);
                black_box(&mut layout).draw(black_box(area), &mut ());
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_column,
    bench_complex_nested,
    bench_deep_nesting,
    bench_wide_layout
);
criterion_main!(benches);
