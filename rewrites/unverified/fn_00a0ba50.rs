// original: 0x00a0ba50 pool_slot_reset (proposed)
/// Reset a small pool slot: clear flag bit 1 at `this + 0x5b` and zero the
/// dwords at +0x4, +0x18 and +0x6c. Thiscall; the return register is
/// untouched by the original.
lf_checker_rt::export!(thiscall, rw_00a0ba50(this: u32) -> u32 {
    unsafe {
        let f = (this + 0x5b) as *mut u8;
        f.write(f.read() & 0xfd);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        ((this + 0x04) as *mut u32).write_unaligned(0);
        ((this + 0x6c) as *mut u32).write_unaligned(0);
        0
    }
});
