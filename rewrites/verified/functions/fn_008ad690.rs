// original: 0x008ad690 audio_clamp_lerp
/// Audio clamped lerp: clamp `x` into `[lo, hi]`, interpolate the outputs.
///
/// At or below `lo` returns the low output, at or above `hi` the high output
/// (unordered comparisons fall through toward the blend); equal outputs
/// short-circuit; otherwise returns the linear blend
/// `(x - lo) / (hi - lo) * (out_hi - out_lo) + out_lo`.
export!(thiscall, rw_008ad690(this_: *const u8, x: f32) -> f32 {
    unsafe {
        let lo = *(this_.add(4) as *const f32);
        let out_lo = *(this_.add(8) as *const f32);
        let hi = *(this_.add(0x0c) as *const f32);
        let out_hi = *(this_.add(0x10) as *const f32);
        if lo >= x {
            return out_lo;
        }
        if x >= hi {
            return out_hi;
        }
        if out_lo == out_hi {
            return out_lo;
        }
        (x - lo) / (hi - lo) * (out_hi - out_lo) + out_lo
    }
});
