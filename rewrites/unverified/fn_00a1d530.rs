// original: 0x00a1d530 cam_mode_weight_test (proposed)

/// Tests whether a mode's weight clears a threshold of 2.0.
///
/// `obj` points to a record with a signed 16-bit mode index at
/// `+MODE_OFF`. The index selects a row from a global pointer table; the
/// row's float at `+WEIGHT_OFF` is compared against `THRESH` (2.0) and
/// the function returns 1 when strictly above it, otherwise 0 (a NaN
/// weight returns 0). Only `al` carries the result.
///
/// Original: 0x00a1d530 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a1d530(obj: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x2e;
        const TABLE: u32 = 0x0129_5cd8;
        const WEIGHT_OFF: u32 = 0x38;
        const THRESH: f32 = f32::from_bits(0x4000_0000); // 2.0
        let idx = ((obj + MODE_OFF) as *const i16).read_unaligned() as isize;
        let base = lf_checker_rt::relocated(TABLE) as *const u32;
        let row = base.offset(idx).read_unaligned();
        let w = f32::from_bits(((row + WEIGHT_OFF) as *const u32).read_unaligned());
        u32::from(w > THRESH)
    }
});
