// original: 0x00E66200 screen_ratio_init_01 (proposed)

/// Divide two global floats and store the quotient in a third global.
///
/// Loads the single-precision values at `0x01038e88` (dividend) and
/// `0x01038e8c` (divisor), divides with one scalar SSE divide, and
/// stores the quotient at `0x01288558`. All three words live in
/// writable game data. The divide runs in the original's operand order with
/// both operands pinned, so quiet-NaN payloads and signed zeros match bit
/// for bit. No calls; EAX is untouched.
///
/// Original: 0x00E66200 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00E66200() -> u32 {
    unsafe {
        const DIVIDEND: u32 = 0x01038e88;
        const DIVISOR: u32 = 0x01038e8c;
        const QUOTIENT: u32 = 0x01288558;
        let a = f32::from_bits(
            (lf_checker_rt::global::<u32>(DIVIDEND) as *const u32).read_unaligned(),
        );
        let b = f32::from_bits(
            (lf_checker_rt::global::<u32>(DIVISOR) as *const u32).read_unaligned(),
        );
        let q = core::hint::black_box(a) / core::hint::black_box(b);
        lf_checker_rt::global::<u32>(QUOTIENT).write_unaligned(q.to_bits());
        0
    }
});
