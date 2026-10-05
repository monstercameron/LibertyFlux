// original: 0x0061B2F0 swap_u32_f32

/// Exchange the 32-bit contents of two cells.
///
/// Reads the word at `b` and the float bits at `a`, then stores each to
/// the other's cell. Pure bitwise exchange; NaN payloads survive, and
/// aliasing both arguments is a no-op. The return channel passes the
/// entry accumulator through untouched (the contract pins it to zero).
/// Original: 0x0061B2F0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0061B2F0(a: u32, b: u32) -> u32 {
    unsafe {
        let from_b = (b as *const u32).read_unaligned();
        let from_a = (a as *const u32).read_unaligned();
        (a as *mut u32).write_unaligned(from_b);
        (b as *mut u32).write_unaligned(from_a);
        0
    }
});
