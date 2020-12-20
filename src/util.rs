pub(crate) fn round(n: f32, decimals: usize) -> f32 {
    let factor = 10_u16.pow(decimals as u32) as f32;
    (n * factor).round() / factor
}
