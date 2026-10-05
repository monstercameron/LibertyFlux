// original: 0x008AD5D0 audio_float_lerp (proposed)

/// Clamp-or-interpolate a value through the four-word range table at `this`.
///
/// `this+0x4` is the low input bound, `this+0xc` the high input bound,
/// `this+0x8` the low output, `this+0x10` the high output. Values at or
/// below the low bound yield the low output, values at or above the high
/// bound the high output; anything strictly inside is linearly interpolated
/// as `(v - lo) / (hi - lo) * (oh - ol) + ol` in the original's SSE operand
/// order, unless the two outputs are bit-equal, in which case the low output
/// is used directly. Unordered (NaN) comparisons fall through to the
/// interpolation, matching the original's `comiss`/`jb` and `ucomiss` flag
/// tests. The result is passed to callee `0x890030` (cdecl, one float word).
/// No return value. Original is thiscall with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008AD5D0(this: u32, v_bits: u32) -> u32 {
    const EMIT: u32 = 1;
    const LO: u32 = 0x4;
    const OL: u32 = 0x8;
    const HI: u32 = 0xc;
    const OH: u32 = 0x10;
    #[inline(always)]
    fn sub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    #[inline(always)]
    fn div(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) / core::hint::black_box(b)
    }
    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    unsafe {
        let lo = f32::from_bits(((this + LO) as *const u32).read_unaligned());
        let ol = f32::from_bits(((this + OL) as *const u32).read_unaligned());
        let hi = f32::from_bits(((this + HI) as *const u32).read_unaligned());
        let oh = f32::from_bits(((this + OH) as *const u32).read_unaligned());
        let v = f32::from_bits(v_bits);
        // `comiss a, b; jb` is taken exactly when `!(a >= b)`, unordered
        // comparisons included; `ucomiss; lahf; (an instruction of the original); jnp` skips the
        // interpolation exactly on ordered equality.
        let out = if !(lo >= v) {
            if !(v >= hi) {
                if ol == oh {
                    ol
                } else {
                    add(mul(div(sub(v, lo), sub(hi, lo)), sub(oh, ol)), ol)
                }
            } else {
                oh
            }
        } else {
            ol
        };
        lf_checker_rt::callee_cdecl!(EMIT, u32, out.to_bits());
    }
    0
});
