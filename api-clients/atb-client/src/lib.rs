wai_bindgen_rust::export!("calculator.wai");
struct Calculator;

impl crate::calculator::Calculator for Calculator {
    fn add(a: f32, b: f32) -> f32 {
        a + b
    }
}
