// original: 0x00B3A010 scaled_index_clamp

/// Scale a global index by a sampled float, clamp it twice, and return the
/// excess over a floor.
///
/// `n = G_IDX + (flag != 0 ? 5 : 0)` where `flag` is the low byte of the
/// argument. Callee 1 (the two-float copier) refreshes a 1.0-initialised
/// stack slot with a scripted float; the product `float(n)` times the slot
/// (or times the scripted word the callee stores through its second
/// destination, the function's own argument slot, when the flag is set) is
/// truncated with `cvttss2si` semantics (NaN and out-of-range yield
/// `0x80000000`, unlike Rust's saturating `as`). The truncation is clamped
/// against two samples from the zero-argument callee 2 (kept while
/// strictly below the first sample, else replaced by the second), then the
/// excess over the `G_FLOOR` global is returned, or 0 when at or below it.
/// Cdecl, one stack word, returns in eax.
///
/// Callee 1's two destinations are a stack slot and the function's own
/// argument slot; both are frame addresses, so both call arguments are
/// skipped and snapshotted, and both stores are scripted. The argument-slot
/// store lands on caller stack and is observed there. Both 1.0 pre-writes
/// are staged (snapshotted at call time) before the callee overwrites them.
///
/// Original: 0x00B3A010.

lf_checker_rt::export!(cdecl, rw_00B3A010(arg: u32) -> u32 {
    unsafe {
        const G_IDX: u32 = 0x0169E410;
        const G_FLOOR: u32 = 0x016624C4;
        const COPY: u32 = 1;
        const SAMPLE: u32 = 2;
        const ONE: u32 = 0x3F800000;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvttss2si(x: f32) -> i32 {
            if -2147483648.0f32 <= x && x < 2147483648.0f32 {
                x as i32
            } else {
                i32::MIN
            }
        }
        let flag = (arg & 0xFF) as u8;
        let mut n = (lf_checker_rt::global::<u32>(G_IDX) as *const u32).read_unaligned() as i32;
        if flag != 0 {
            n = n.wrapping_add(5);
        }
        let mut slot: u32 = ONE;
        // 1.0 pre-write like the original (snapshotted at call time).
        core::hint::black_box((&arg as *const u32) as *mut u32).write_unaligned(ONE);
        let _: u32 = lf_checker_rt::callee_cdecl!(COPY, u32, &mut slot as *mut u32 as u32, &arg as *const u32 as u32);
        let other = if flag != 0 {
            f32::from_bits(arg)
        } else {
            f32::from_bits(slot)
        };
        let mut v = cvttss2si(mul(n as f32, other));
        let s1 = lf_checker_rt::callee_cdecl!(SAMPLE, u32,) as i32;
        if !(v < s1) {
            v = lf_checker_rt::callee_cdecl!(SAMPLE, u32,) as i32;
        }
        let floor = (lf_checker_rt::global::<u32>(G_FLOOR) as *const u32).read_unaligned() as i32;
        if floor < v {
            v.wrapping_sub(floor) as u32
        } else {
            0
        }
    }
});
