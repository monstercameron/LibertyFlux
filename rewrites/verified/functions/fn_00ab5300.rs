// original: 0x00ab5300 stream_time_accumulate (proposed)

/// Advance a streaming clock and report whether its window elapsed.
///
/// Adds `delta` to the clock word `*clock`, then tests the deadline:
/// with `C = 1.15f` and `t = (*clock_old + C) - limit`, returns 1 exactly
/// when `t > -0.15f` (never, when `t` is NaN) and `C > t`, else 0. The
/// float operation order is the original's.
///
/// Original: 0x00ab5300 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00ab5300(delta: u32, clock: u32, limit: u32) -> u32 {
    unsafe {
        const WINDOW: f32 = f32::from_bits(0x3F93_3333); // 1.15
        const FLOOR: f32 = f32::from_bits(0xBE19_999A); // -0.15
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let old = f32::from_bits((clock as *const u32).read_unaligned());
        let t = sub(add(old, WINDOW), f32::from_bits(limit));
        let hit = u32::from(t > FLOOR && WINDOW > t);
        let new = add(old, f32::from_bits(delta));
        (clock as *mut u32).write_unaligned(new.to_bits());
        hit
    }
});
