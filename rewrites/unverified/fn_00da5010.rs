// original: 0x00DA5010 task_range_probe (proposed)

/// Range probe: decide whether the task at `this` needs its far check and
/// run it through the probe callee when it does.
///
/// The flag byte at `+0x46` and the mode word at `+0x48` select the path:
/// a set flag with a set mode fires at once, a set flag with a clear mode
/// (or the reverse) returns 0, and a clear flag with a clear mode fires
/// only when the squared length of (`+0x30`, `+0x34`, `+0x38`) exceeds the
/// read-only threshold. The probe call forwards the caller argument, a
/// zero, the mode word, the vector address, the word at `+0x20` and the
/// constant 3.0; its low byte is the result. Float arithmetic runs in the
/// original's order through order-pinning helpers, and the unordered
/// (NaN) case takes the same path as below-or-equal on both sides.
/// Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da5010(this: u32, arg: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const FLAG_OFF: u32 = 0x46;
        const MODE_OFF: u32 = 0x48;
        const VEC_OFF: u32 = 0x30;
        const THRESH_SLOT: u32 = 0x00FE876C;
        const WEIGHT: u32 = 0x40400000;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let flag = ((this + FLAG_OFF) as *const u8).read();
        let mode = ((this + MODE_OFF) as *const u32).read();
        let fire = if flag != 0 {
            mode != 0
        } else if mode != 0 {
            false
        } else {
            let x = ((this + VEC_OFF) as *const f32).read();
            let y = ((this + VEC_OFF + 4) as *const f32).read();
            let z = ((this + VEC_OFF + 8) as *const f32).read();
            let sq = add(add(mul(x, x), mul(y, y)), mul(z, z));
            let thresh = (lf_checker_rt::relocated(THRESH_SLOT) as *const f32).read();
            sq > thresh
        };
        if fire {
            let w20 = ((this + 0x20) as *const u32).read();
            lf_checker_rt::callee_cdecl!(PROBE, u32, arg, 0, mode, this + VEC_OFF, w20, WEIGHT)
        } else {
            0
        }
    }
});
