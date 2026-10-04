// original: 0x0069a6b0 rage::crAnimChannelRawFloat::vf13
/// Same interpolation as vf9 but returned in ST0 (as `f64`, exactly
/// representing the `f32` result). The trailing stack argument is unused; the
/// original instead scribbles its low stack slot as a float temporary, so the
/// stack channel is unchecked for this function.
export!(thiscall, rw_0069a6b0(this: u32, idx: u32, t: f32, _unused: u32) -> f64 {
    unsafe {
        let keys = *((this + 8) as *const u32);
        let stride = idx.wrapping_mul(4);
        let a = *((keys + stride) as *const f32);
        let b = *((keys + stride + 4) as *const f32);
        ((b - a) * t + a) as f64
    }
});
