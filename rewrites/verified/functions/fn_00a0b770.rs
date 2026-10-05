// original: 0x00a0b770 pool_handle_state_reset (proposed)
/// Reset the cached state of a pool-handle wrapper.
///
/// Clears flag bit 0 at `this + 0x5b` and zeroes the dwords at +0x40, +0x44,
/// +0x48 and +0x390. Returns `this`. Thiscall.
lf_checker_rt::export!(thiscall, rw_00a0b770(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x5b;
        const FLAG_KEEP: u8 = 0xfe;
        let f = (this + FLAG_OFF) as *mut u8;
        f.write(f.read() & FLAG_KEEP);
        for off in [0x390u32, 0x48, 0x44, 0x40] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        this
    }
});
