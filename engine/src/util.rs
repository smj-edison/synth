pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    (end - start) * amount + start
}
