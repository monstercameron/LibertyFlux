// original: 0x00CAB7A0 event_forward_construct (proposed)

/// Construct a forwarded event in place by delegating every argument.
///
/// Forwards all seven stack arguments unchanged to the seven-argument worker
/// on `this` (the original shuffles them through floating-point registers,
/// which preserves their bits), then installs this record's virtual table
/// and returns `this`.
///
/// Original: 0x00CAB7A0 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_00cab7a0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, f4_bits: u32, f5_bits: u32, a6: u32) -> u32 {
    unsafe {
        const WORKER: u32 = 1;
        const VTABLE: u32 = 0x00ED8324;
        lf_checker_rt::callee_thiscall!(WORKER, u32, this, a0, a1, a2, a3, f4_bits, f5_bits, a6);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
