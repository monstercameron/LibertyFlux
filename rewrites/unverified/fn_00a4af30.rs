// original: 0x00A4AF30 vehicle_pick_nonzero_slot (proposed)

/// Returns the first non-zero word of the eight at `this + SLOTS`, starting
/// the scan at a callee-derived index.
///
/// Calls the id callee (no stack arguments, `this` in `ecx`), takes its
/// answer modulo 2^16 as an exact float, scales it by `STEP` (2^-15) and
/// `SPAN` (8.0) in that order, truncates to an integer start index (always in
/// 0..16: no overflow, no saturation), then scans at most eight slots from
/// `(trial + start) mod 8` and returns the first non-zero word, or 0 when all
/// eight are zero. Float order is pinned with `black_box`.
///
/// Original: 0x00A4AF30 (thiscall, no stack words), one callee, float scale.
lf_checker_rt::export!(thiscall, rw_00A4AF30(this: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x0F54;
        const COUNT: i32 = 8;
        const ID_CALLEE: u32 = 1;
        const STEP: f32 = f32::from_bits(0x3800_0000); // 2^-15
        const SPAN: f32 = f32::from_bits(0x4100_0000); // 8.0
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let raw: u32 = lf_checker_rt::callee_thiscall!(ID_CALLEE, u32, this);
        let scaled = mul(mul((raw & 0xFFFF) as f32, STEP), SPAN);
        let start = scaled as i32;
        let mut i: i32 = 0;
        while i < COUNT {
            let c = (i + start).rem_euclid(COUNT);
            let slot =
                ((this + SLOTS + (c as u32) * 4) as *const u32).read_unaligned();
            if slot != 0 {
                return slot;
            }
            i += 1;
        }
        0
    }
});
