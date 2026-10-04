// original: 0x008eaf50 int_to_shifted_float
/// Scale an integer into a shifted float on the x87 stack.
///
/// Returns `(a * K1 - K2)` in ST0.
export!(stdcall, rw_008eaf50(a: i32) -> f64 {
    unsafe {
        let v = a as f32 * *global::<f32>(0xE83174)
            - *global::<f32>(0xFE8C78);
        v as f64
    }
});
