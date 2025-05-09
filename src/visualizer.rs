// use crate::parser::Expression;
// use animaterm::prelude::*;
// use std::time::Duration;
//
// fn visualize(list: Vec<Expression>) {
//     let capture_keyboard = true;
//     let cols = Some(40);
//     let rows = None; // use all rows available
//     let glyph = Some(Glyph::default()); // initially fill the screen with this
//                                         // You can crank refresh_timeout down, but anything below 1ms won't make a difference,
//                                         // other than high CPU usage.
//                                         // With default 30ms you get as high as 33 FPS, probably enough for a terminal application.
//     let refresh_timeout = Some(Duration::from_milis(10));
//     let mut mgr = Manager::new(capture_keyboard, cols, rows, glyph, refresh_timeout);
//
//     let mut keep_running = true;
//     let mut i = 0;
//     while keep_running {
//         if let Some(key) = mgr.read_key() {
//             match key {
//                 Key::Q | Key::ShiftQ => {
//                     keep_running = false;
//                 }
//                 Key::Right | Key::Space | Key::E | Key::Tab => {
//                     print_expression(&list[i], mgr);
//                     i += 1;
//                 }
//                 _ => continue,
//             }
//         }
//     }
//     mgr.terminate();
// }
//
// fn expr_to_visual(expr: &Expression) -> String {
//     match &expr {
//         Expression::Abstraction(id, sub_expr) => {
//             let sub = expr_to_visual(sub_expr);
//
//             format!("████████\n{}", sub)
//         }
//         Expression::Application(id, )
//     }
// }
