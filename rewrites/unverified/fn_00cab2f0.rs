// original: 0x00CAB2F0 handler_state_clear (proposed)

/// Clear a handler state block and return its address.
///
/// Writes a zero dword at `+0x00` and zero bytes at `+0x04`, `+0x24` and the
/// fourteen flag slots from `+0x3C` to `+0x1AC` in steps of 0x20 (the
/// original writes that flag range twice; the second pass is redundant).
/// Returns `this` in eax. No calls.
///
/// Original: 0x00CAB2F0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cab2f0(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const FLAG_FIRST: u32 = 0x3C;
        const FLAG_LAST: u32 = 0x1AC;
        const FLAG_STEP: u32 = 0x20;
        (this.wrapping_add(HEAD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x04) as *mut u8).write(0);
        (this.wrapping_add(0x24) as *mut u8).write(0);
        let mut off = FLAG_FIRST;
        while off <= FLAG_LAST {
            (this.wrapping_add(off) as *mut u8).write(0);
            off += FLAG_STEP;
        }
        this
    }
});
