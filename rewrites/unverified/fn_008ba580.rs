// original: 0x008BA580 frontend_clear_pair_flag

/// Clear the frontend pair flag bytes when set.
///
/// When the flag byte at `FLAG` is non-zero, writes zero to `FLAG` and to
/// the neighbouring byte at `FLAG_NEXT`; otherwise leaves memory alone.
/// Leaf, cdecl, no arguments; no meaningful return value.
lf_checker_rt::export!(cdecl, rw_008BA580() -> u32 {
    unsafe {
        const FLAG: u32 = 0x0116_0C2A;
        const FLAG_NEXT: u32 = 0x0116_0C2B;
        let p = lf_checker_rt::relocated(FLAG) as *mut u8;
        if p.read_unaligned() != 0 {
            p.write_unaligned(0);
            (lf_checker_rt::relocated(FLAG_NEXT) as *mut u8).write_unaligned(0);
        }
        0
    }
});
