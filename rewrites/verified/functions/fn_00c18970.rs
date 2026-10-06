// original: 0x00c18970 replay_struct_init

/// Initialise a replay-channel record with constant dimensions and limits.
///
/// Stores `0x280, 0x1e0, 0x1000, 0xbb8, 0x1e` at `+0x0..+0x10` of `this`,
/// zeroes the two qwords at `+0x14`/`+0x1c`, the flag byte at `+0x24` and the
/// three pointer slots at `+0x28/+0x2c/+0x30`, then calls the channel
/// bring-up helper (thiscall, no stack arguments) and returns `this`.
///
/// Original: 0x00C18970 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c18970(this: u32) -> u32 {
    unsafe {
        const BRING_UP: u32 = 1;
        ((this) as *mut u32).write_unaligned(0x280);
        ((this + 4) as *mut u32).write_unaligned(0x1e0);
        ((this + 8) as *mut u32).write_unaligned(0x1000);
        ((this + 0x0c) as *mut u32).write_unaligned(0xbb8);
        ((this + 0x10) as *mut u32).write_unaligned(0x1e);
        ((this + 0x14) as *mut u64).write_unaligned(0);
        ((this + 0x1c) as *mut u64).write_unaligned(0);
        ((this + 0x24) as *mut u8).write(0);
        ((this + 0x28) as *mut u32).write_unaligned(0);
        ((this + 0x2c) as *mut u32).write_unaligned(0);
        ((this + 0x30) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(BRING_UP, u32, this);
        this
    }
});
