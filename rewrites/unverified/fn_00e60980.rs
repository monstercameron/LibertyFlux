// original: 0x00E60980 qpc_micros_init (proposed)

/// Timer-scale initialisation from the performance counter.
///
/// Behaviour: calls the counter reader (cdecl, one ignored argument 0;
/// 64-bit answer in EDX:EAX), then calls the scaler (stdcall) with
/// `(counter_lo, counter_hi, 1_000_000, 0)` and stores its 64-bit answer
/// to the globals `SCALE_LO`/`SCALE_HI` (low word first). Returns the
/// scaler's 64-bit answer in EDX:EAX.
///
/// Original: cdecl, no stack arguments; the counter halves travel in
/// registers (pushed straight onto the scaler's argument slots).
/// Edge cases: a zero high half takes the multiply path in the real
/// scaler; both halves are stubbed here, so every 64-bit input shape is
/// exercised through the script instead.
lf_checker_rt::export!(cdecl, rw_00e60980() -> u64 {
    unsafe {
        const MICROS_PER_SEC: u32 = 1_000_000;
        let stamp: u64 = lf_checker_rt::callee_cdecl!(1, u64, 0);
        let lo = stamp as u32;
        let hi = (stamp >> 32) as u32;
        let scaled: u64 =
            lf_checker_rt::callee_stdcall!(2, u64, lo, hi, MICROS_PER_SEC, 0);
        (lf_checker_rt::relocated(0x0019F3A60) as *mut u32).write_unaligned(scaled as u32);
        (lf_checker_rt::relocated(0x0019F3A64) as *mut u32)
            .write_unaligned((scaled >> 32) as u32);
        scaled
    }
});

