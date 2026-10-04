// original: 0x0069a680 rage::crAnimChannelRawFloat::vf9
/// Interpolate between raw keyframes `idx` and `idx+1` at fraction `t` and
/// store `(b - a) * t + a` at `out`. Returns `out`.
export!(thiscall, rw_0069a680(this: u32, idx: u32, t: f32, out: u32) -> u32 {
    unsafe {
        let keys = *((this + 8) as *const u32);
        let stride = idx.wrapping_mul(4);
        let a = *((keys + stride) as *const f32);
        let b = *((keys + stride + 4) as *const f32);
        *(out as *mut f32) = (b - a) * t + a;
        out
    }
});
