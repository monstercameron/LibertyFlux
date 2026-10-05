// original: 0x00B39FE0 copy_two_global_floats

/// Copy two engine-global tuning floats to two caller out-pointers.
///
/// Reads the globals at `G_FLOAT0`/`G_FLOAT1` and stores them to `dst0`/`dst1`.
/// The moves are bitwise (the original uses `movss` as a copy). Cdecl, two
/// stack words, no meaningful return value.
///
/// Original: 0x00B39FE0.

lf_checker_rt::export!(cdecl, rw_00B39FE0(dst0: u32, dst1: u32) -> u32 {
    unsafe {
        const G_FLOAT0: u32 = 0x0104592C;
        const G_FLOAT1: u32 = 0x01045930;
        let v0 = (lf_checker_rt::global::<u32>(G_FLOAT0) as *const u32).read_unaligned();
        (dst0 as *mut u32).write_unaligned(v0);
        let v1 = (lf_checker_rt::global::<u32>(G_FLOAT1) as *const u32).read_unaligned();
        (dst1 as *mut u32).write_unaligned(v1);
        0
    }
});
