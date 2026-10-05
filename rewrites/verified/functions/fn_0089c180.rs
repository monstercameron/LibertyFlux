// original: 0x0089C180 aud_loop_level_update (proposed)

/// Refresh the loop level of an audio sound from its counters and helpers.
///
/// `this` is an audio sound object. Two unsigned counters (at `+0xB4` and
/// `+0xB8`) are widened to floats and each is raised by an optional live
/// level: the pointers at `+0xC4`/`+0xC8`, when non-null, point at floats
/// that are added (`memory + counter`, the original's operand order). The
/// mixer helper (callee 0, cdecl) combines the two levels into one float;
/// the result is clamped to a gain of `1.0 / result` when it exceeds 1
/// (an unordered comparison result keeps 1, matching `comiss`+`jbe`) and
/// handed twice to the gain helper (callee 1, `thiscall` on `this+0xDC`).
/// The object level at `+0xD4` becomes `1 - old`, two more optional
/// pointers (`+0xCC`, `+0xD0`) each contribute a truncated-to-int float
/// (`cvttss2si` semantics, low 16 bits) to accumulators seeded from
/// `+0xBC`/`+0xC0`, the accumulator helper (callee 2, cdecl) combines
/// them, and the incoming stack argument is added: the sum is stored at
/// `+0xB0` and returned. All integer work is exact; float order is the
/// original's.
///
/// Original: 0x0089C180 (thiscall, one stack word, returns full `eax`).
lf_checker_rt::export!(thiscall, rw_0089C180(this: u32, arg: u32) -> u32 {
    unsafe {
        const COUNT_A: u32 = 0xB4;
        const COUNT_B: u32 = 0xB8;
        const LEVEL_A_PTR: u32 = 0xC4;
        const LEVEL_B_PTR: u32 = 0xC8;
        const TRIM_A_PTR: u32 = 0xCC;
        const TRIM_B_PTR: u32 = 0xD0;
        const ACC_A: u32 = 0xBC;
        const ACC_B: u32 = 0xC0;
        const LEVEL: u32 = 0xD4;
        const RESULT: u32 = 0xB0;
        const GAIN_OBJ: u32 = 0xDC;
        const ONE: f32 = 1.0;
        const CALLEE_MIX: u32 = 0;
        const CALLEE_GAIN: u32 = 1;
        const CALLEE_ACC: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Bit-exact `cvttss2si`: truncate toward zero; NaN, infinity and
        /// out-of-range magnitudes (except exactly -2^31) give 0x80000000.
        fn trunc_to_i32(bits: u32) -> i32 {
            let exp = ((bits >> 23) & 0xFF) as i32;
            let mant = bits & 0x7FFF_FF;
            if exp == 0xFF {
                return i32::MIN;
            }
            let e = exp - 127;
            if e < 0 {
                return 0;
            }
            if e >= 31 {
                return i32::MIN;
            }
            let m = (1u32 << 23) | mant;
            let v: u64 = if e >= 23 { (m as u64) << (e - 23) } else { (m >> (23 - e)) as u64 };
            if bits & 0x8000_0000 == 0 {
                if v >= 0x8000_0000 { i32::MIN } else { v as i32 }
            } else if v > 0x8000_0000 {
                i32::MIN
            } else if v == 0x8000_0000 {
                i32::MIN
            } else {
                -(v as i32)
            }
        }

        // Counter A as a float, plus its optional live level.
        let mut level_a = rd32(this + COUNT_A) as f32;
        let pa = rd32(this + LEVEL_A_PTR);
        if pa != 0 {
            level_a = add(rdf(pa), level_a);
        }
        // Counter B likewise.
        let mut level_b = rd32(this + COUNT_B) as f32;
        let pb = rd32(this + LEVEL_B_PTR);
        if pb != 0 {
            level_b = add(rdf(pb), level_b);
        }
        let mixed: f32 = lf_checker_rt::callee_cdecl!(CALLEE_MIX, f32, level_a.to_bits(), level_b.to_bits());
        // Gain: 1/mixed while mixed is strictly above 1, else 1.
        let gain = if mixed > ONE {
            core::hint::black_box(ONE) / core::hint::black_box(mixed)
        } else {
            ONE
        };
        lf_checker_rt::callee_thiscall!(CALLEE_GAIN, u32, this + GAIN_OBJ, gain.to_bits(), gain.to_bits());
        wr32(this + LEVEL, sub(ONE, rdf(this + LEVEL)).to_bits());
        // Trims folded into the accumulators.
        let mut acc_a = rd32(this + ACC_A);
        let ta = rd32(this + TRIM_A_PTR);
        if ta != 0 {
            acc_a = acc_a.wrapping_add((trunc_to_i32(rd32(ta)) as u16) as u32);
        }
        let mut acc_b = rd32(this + ACC_B);
        let tb = rd32(this + TRIM_B_PTR);
        if tb != 0 {
            acc_b = acc_b.wrapping_add((trunc_to_i32(rd32(tb)) as u16) as u32);
        }
        let total: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ACC, u32, acc_a, acc_b);
        let out = total.wrapping_add(arg);
        wr32(this + RESULT, out);
        out
    }
});
