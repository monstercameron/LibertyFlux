// original: 0x00E666F0 div_consts_store_0128e3f0 (proposed)

/// Divide two constant floats and store the quotient in its global.
///
/// Reads the dividend and divisor from the constant table, divides in
/// single precision with the original's operand order (pinned against
/// reordering), and writes the quotient to the destination global.
/// Division by zero yields infinity and zero over zero a NaN, exactly as
/// the single-precision divide instruction does. No arguments, no calls;
/// eax is untouched so the function returns nothing (cdecl).
lf_checker_rt::export!(cdecl, rw_00e666f0() -> u32 {
    unsafe {
        const DIVIDEND: u32 = 0x0103928C;
        const DIVISOR: u32 = 0x01039290;
        const DEST: u32 = 0x0128E3F0;
        let a = f32::from_bits((lf_checker_rt::global::<u32>(DIVIDEND) as *const u32).read_unaligned());
        let b = f32::from_bits((lf_checker_rt::global::<u32>(DIVISOR) as *const u32).read_unaligned());
        let q = core::hint::black_box(a) / core::hint::black_box(b);
        (lf_checker_rt::global::<u32>(DEST) as *mut u32).write_unaligned(q.to_bits());
        0
    }
});
