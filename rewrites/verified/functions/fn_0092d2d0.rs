// original: 0x0092D2D0 clamped_shift_from_global (proposed)

/// Derive a clamped power-of-two limit from a global count.
///
/// Reads the signed dword at `GLOBAL`. When it is zero or negative the
/// result is 0. Otherwise the result is `0x80 << count` (the shift count is
/// the low 5 bits, as on x86), clamped to `MAX`: when the shifted value is
/// *signed*-greater than `MAX` it is replaced by `MAX` (the original uses a
/// signed `cmovg`, so values with the high bit set, e.g. count 24, are kept
/// as-is).
///
/// Original: 0x0092D2D0 (cdecl, no arguments). Leaf: no calls, no writes.
lf_checker_rt::export!(cdecl, rw_0092D2D0() -> u32 {
    unsafe {
        const GLOBAL: u32 = 0x0116_0EAC;
        const BASE: u32 = 0x80;
        const MAX: u32 = 0x800;
        let count = (lf_checker_rt::relocated(GLOBAL) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let shifted = BASE.wrapping_shl(count as u32);
        if (shifted as i32) > (MAX as i32) {
            MAX
        } else {
            shifted
        }
    }
});
