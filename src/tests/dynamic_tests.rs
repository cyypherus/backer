#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    #[test]
    fn test_simple() {
        Layout::new({
            column(vec![
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 0., 100., 50.));
                })
                .dynamic_height(|w, _| w * 0.5),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 50., 100., 50.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_nested() {
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 20., 100., 50.));
                })
                .dynamic_height(|w, _| w * 0.5)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 70., 100., 10.));
                })
                .height(10.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new({
            column(vec![
                row(vec![draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., -5., 100., 50.));
                })
                .dynamic_height(|w, _| w * 0.5)]),
                draw(|a, _: &mut ()| {
                    assert_eq!(a, Area::new(0., 45., 100., 60.));
                })
                .height(60.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
}
