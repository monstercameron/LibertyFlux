// original: 0x00E609C0 qpc_seed_hash_init (proposed)

/// Timer-seed hash initialisation from the performance counter.
///
/// Behaviour: calls the counter reader (cdecl, one ignored argument 0;
/// only the low 32 bits are used), then folds them: `flag` is 1 when the
/// counter is zero else 0; `mix = counter.rotate_left(16) ^ counter`;
/// `acc = flag + counter` (wrapping); `prod = acc * MAGIC` (64-bit);
/// `eax = prod.lo + mix`, `edx = prod.hi` (wrapping, carry out kept in
/// the addition as the original's `adc`). Stores EAX to `HASH_LO` and
/// EDX to `HASH_HI`, preserves ESI, and returns EAX.
///
/// `MAGIC` is 0x5CDCFAA7. All arithmetic wraps; the zero-counter edge
/// yields `flag = 1`, every other input `flag = 0`.
///
/// Original: cdecl, no stack arguments; callee-saved ESI preserved.
lf_checker_rt::export!(cdecl, rw_00e609c0() -> u32 {
    unsafe {
        const MIXER: u32 = 0x5CDCFAA7;
        let counter: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0);
        let flag: u32 = if counter == 0 { 1 } else { 0 };
        let mix: u32 = counter.rotate_left(16) ^ counter;
        let acc: u32 = flag.wrapping_add(counter);
        let prod: u64 = (acc as u64).wrapping_mul(MIXER as u64);
        let (lo, carry) = (prod as u32).overflowing_add(mix);
        let hi: u32 = ((prod >> 32) as u32).wrapping_add(carry as u32);
        (lf_checker_rt::relocated(0x0019F7F24) as *mut u32).write_unaligned(lo);
        (lf_checker_rt::relocated(0x0019F7F28) as *mut u32).write_unaligned(hi);
        lo
    }
});

