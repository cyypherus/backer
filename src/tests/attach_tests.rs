// #[cfg(test)]
// mod tests {
//     use crate::layout::*;
//     use crate::models::*;
//     use crate::nodes::*;
//     #[test]
//     fn test_attach() {
//         Layout::new({
//             stack(vec![
//                 //>
//                 draw(move |a, _: &mut ()| {
//                     assert_eq!(a, Area::new(40., 40., 20., 20.));
//                 })
//                 .width(20.)
//                 .height(20.)
//                 .pad(10.)
//                 .attach_under(draw(|a, _: &mut ()| {
//                     assert_eq!(a, Area::new(30., 30., 40., 40.));
//                 })),
//             ])
//         })
//         .draw(Area::new(0., 0., 100., 100.), &mut ());
//         Layout::new({
//             stack(vec![draw(|a, _: &mut ()| {
//                 assert_eq!(a, Area::new(40., 40., 20., 20.));
//             })
//             .width(20.)
//             .height(20.)
//             .pad(10.)
//             .attach_over(draw(|a, _: &mut ()| {
//                 assert_eq!(a, Area::new(30., 30., 40., 40.));
//             }))])
//         })
//         .draw(Area::new(0., 0., 100., 100.), &mut ());
//     }
// }
