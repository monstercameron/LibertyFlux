// original: 0x00a9fb10 stream_clear_and_release (proposed)

/// Clear all 256 entries, unregister each, then release the shared pool.
///
/// `this` points to the table object. Every entry of 0x1c bytes has its
/// words at `+0x08` and `+0x0c` and its bytes at `+0x18` and `+0x1a`
/// zeroed, and is passed (base address on the stack) to the unregister
/// routine with `this + 0x1c00` in ECX. Afterwards the pool release routine
/// is tail-called with file address 0x01723bb0 in ECX, and its answer is
/// the result.
///
/// Original: 0x00a9fb10 (thiscall, no stack arguments; ends in a tail jump;
/// 257 outgoing calls, so the contract raises the call-log cap).
lf_checker_rt::export!(thiscall, rw_00a9fb10(this: u32) -> u32 {
    unsafe {
        const ENTRY_STRIDE: u32 = 0x1c;
        const ENTRY_COUNT: u32 = 0x100;
        const UNREG_THIS_OFF: u32 = 0x1c00;
        const POOL: u32 = 0x01723bb0;
        const UNREGISTER: u32 = 1;
        const RELEASE: u32 = 2;
        let mut i = 0u32;
        while i < ENTRY_COUNT {
            let e = this.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
            ((e + 8) as *mut u32).write_unaligned(0);
            ((e + 0x0c) as *mut u32).write_unaligned(0);
            ((e + 0x18) as *mut u8).write(0);
            ((e + 0x1a) as *mut u8).write(0);
            lf_checker_rt::callee_thiscall!(
                UNREGISTER,
                u32,
                this.wrapping_add(UNREG_THIS_OFF),
                e
            );
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(RELEASE, u32, lf_checker_rt::relocated(POOL))
    }
});
