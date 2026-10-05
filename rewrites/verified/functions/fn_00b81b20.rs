// original: 0x00B81B20 thread_state_reset
/// Reset a script thread's run-state fields to their idle values.
///
/// Clears the low two flag bits at `FLAGS`, zeroes the pending fields at
/// `P0`..`P2`, the handle at `HDL` and the bytes at `B0`/`B1`, and installs
/// the constants `C94`, `C98`, `C9C` and `C9E`.
///
/// Original: 0x00B81B20 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B81B20(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const FLAGS: u32 = 0xAC;
        const B0: u32 = 0x70;
        const P0: u32 = 0x88;
        const P1: u32 = 0x8C;
        const P2: u32 = 0x90;
        const C94: u32 = 0x94;
        const C98: u32 = 0x98;
        const C9C: u32 = 0x9C;
        const C9E: u32 = 0x9E;
        const HDL: u32 = 0xA0;
        const B1: u32 = 0xA4;
        wr32(this + FLAGS, rd32(this + FLAGS) & 0xFFFF_FFFC);
        wr8(this + B0, 0);
        wr32(this + P0, 0);
        wr32(this + P1, 0);
        wr32(this + P2, 0);
        wr32(this + C94, 0x0100_0000);
        wr32(this + C98, 1);
        unsafe { ((this + C9C) as *mut u16).write_unaligned(0x0100) }
        wr8(this + C9E, 0xFF);
        wr32(this + HDL, 0);
        wr8(this + B1, 0);
        0
    }
});
