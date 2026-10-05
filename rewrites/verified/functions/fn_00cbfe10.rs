// original: 0x00CBFE10 task_state_init_defaults (proposed)

/// Initialise a ped-task state block with its default values.
///
/// `this` points to the task object; the words at `+0x54..0xcc` are filled
/// with constants (two 3.0 floats, one 1.0 float, a 0x1c tag, -1 sentinels
/// and zeroes), one word is copied from a configuration global, bit 0 of
/// the flag word at `+0xa8` is cleared, and the function returns 0.
///
/// Original: 0x00CBFE10 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cbfe10(this: u32) -> u32 {
    unsafe {
        const THREE: u32 = 0x4040_0000; // 3.0f
        const ONE: u32 = 0x3f80_0000; // 1.0f
        const CFG_GLOBAL: u32 = 0x0117_35b4;
        const TAG: u32 = 0x1c;

        #[inline(always)]
        unsafe fn wr32(base: u32, off: u32, v: u32) {
            unsafe { (base.wrapping_add(off) as *mut u32).write_unaligned(v) }
        }

        let t = this;
        wr32(t, 0x54, THREE);
        wr32(t, 0x58, 0);
        wr32(t, 0x5c, 0);
        wr32(t, 0x60, 0);
        wr32(t, 0x64, 0);
        wr32(t, 0x6c, ONE);
        wr32(t, 0x70, THREE);
        wr32(t, 0x74, 0);
        wr32(t, 0x78, 0);
        wr32(t, 0x7c, 0);
        let cfg: u32 = unsafe { lf_checker_rt::global::<u32>(CFG_GLOBAL).read() };
        wr32(t, 0x80, cfg);
        wr32(t, 0x84, TAG);
        wr32(t, 0x88, 0xffff_ffff);
        wr32(t, 0x8c, 0);
        wr32(t, 0x90, 0);
        wr32(t, 0x98, 0);
        wr32(t, 0x9c, 0);
        wr32(t, 0xa0, 0);
        wr32(t, 0xa4, 0);
        let flags = unsafe { (t.wrapping_add(0xa8) as *const u16).read_unaligned() };
        unsafe { (t.wrapping_add(0xa8) as *mut u16).write_unaligned(flags & 0xfffe) };
        unsafe { (t.wrapping_add(0xaa) as *mut u16).write_unaligned(0) };
        wr32(t, 0xac, 0xffff_ffff);
        wr32(t, 0xb0, 0xffff_ffff);
        wr32(t, 0xb4, 0xffff_ffff);
        wr32(t, 0xb8, 0xffff_ffff);
        wr32(t, 0xbc, 0xffff_ffff);
        wr32(t, 0xc0, 0xffff_ffff);
        wr32(t, 0xc4, 0xffff_ffff);
        wr32(t, 0xc8, 0xffff_ffff);
        wr32(t, 0xcc, 0xffff_ffff);
    }
    0
});
