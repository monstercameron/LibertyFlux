// original: 0x008B9050 frontend_options_pair_changed

/// Report whether the two frontend option words differ.
///
/// Reads the words at `FIRST` and `SECOND` and returns 1 when they differ,
/// 0 when they are equal. Leaf, cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_008B9050() -> u32 {
    unsafe {
        const FIRST: u32 = 0x0116_0CC8;
        const SECOND: u32 = 0x0116_0CCC;
        let a = (lf_checker_rt::relocated(FIRST) as *const u32).read_unaligned();
        let b = (lf_checker_rt::relocated(SECOND) as *const u32).read_unaligned();
        u32::from(a != b)
    }
});
