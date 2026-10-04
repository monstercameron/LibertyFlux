// original: 0x00afe370 clamp_global_float
/// Step the shared level down by a fixed amount and clamp it into [0, cap].
/// The comparisons mirror comiss+ja/jbe exactly, including NaN behaviour.
export!(cdecl, rw_00afe370() -> u32 {
    unsafe {
        let slot = global::<f32>(0x103ffec);
        let value = *slot - *global::<f32>(0xfe879c);
        let cap = *global::<f32>(0xfe88e8);
        let clamped = if !(cap > value) {
            cap
        } else if !(0.0 > value) {
            value
        } else {
            0.0
        };
        *slot = clamped;
        0
    }
});
