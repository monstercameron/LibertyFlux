// original: 0x00699970 rage::crAnimChannelQuantizeFloat::vf13
/// Evaluate a quantized-float channel: scale, bias and interpolate.
///
/// Converts the two unsigned 64-bit samples to float, applies the scale at
/// offset 0x14 and bias at 0x18 to each, and returns
/// `(scaled1 - scaled0) * t + scaled0` for the factor `t`. The first stack
/// argument is never read; the pointer's own slot doubles as the original's
/// f32 scratch after the load. Returned as f64 so the value travels in ST0
/// like the original.
export!(thiscall, rw_00699970(this: u32, _ignored: u32, t: f32, p: u32) -> f64 {
    unsafe {
        let scale = *((this as *const f32).add(5));
        let bias = *((this as *const f32).add(6));
        let a = (((*((p as *const u32).add(1))) as u64) << 32)
            | (*(p as *const u32) as u64);
        let b = (((*((p as *const u32).add(3))) as u64) << 32)
            | (*((p as *const u32).add(2)) as u64);
        let v0 = (a as f32) * scale + bias;
        let v1 = (b as f32) * scale + bias;
        ((v1 - v0) * t + v0) as f64
    }
});
