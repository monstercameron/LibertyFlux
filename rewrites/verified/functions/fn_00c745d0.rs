// original: 0x00c745d0 CPedModelInfo::vf1

/// Reset a ped model info object to its default state (virtual slot 1).
///
/// `this` points to the object (at least 0x15E bytes). The routine first runs
/// the base initialiser (intercepted callee: thiscall, no stack arguments),
/// then clears two flag bits at `+0x158` (bit 0, later bit 1), zeroes some
/// twenty fields, writes "none" markers (-1) at `+0x70`, `+0x78`, `+0x80`,
/// `+0x84`, 16-bit "none" markers at `+0x98`, `+0xB0`, `+0x15A`, `+0x15C`,
/// default tuning values at `+0xEC`, `+0xF4` (0.8), `+0xF8` (1.3), `+0xFC`
/// (50.0), and the count 2 at `+0x120`. Returns -1 (the marker left in the
/// accumulator).
///
/// Original: 0x00C745D0 (thiscall, no stack arguments; returns 0xFFFFFFFF).
lf_checker_rt::export!(thiscall, rw_00C745D0(this: u32) -> u32 {
    unsafe {
        const BASE_INIT: u32 = 1;
        const NONE32: u32 = 0xFFFF_FFFF;
        const NONE16: u16 = 0xFFFF;
        const FLAGS: u32 = 0x158;
        const TUNING_PACK: u32 = 0x0302_021E;
        const WALK_BLEND: u32 = 0x3F4C_CCCD; // 0.8
        const RUN_BLEND: u32 = 0x3FA6_6666; // 1.3
        const DRAW_DIST: u32 = 0x4248_0000; // 50.0

        #[inline(always)]
        unsafe fn w32(base: u32, off: u32, val: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(val) }
        }
        #[inline(always)]
        unsafe fn w16(base: u32, off: u32, val: u16) {
            unsafe { ((base + off) as *mut u16).write_unaligned(val) }
        }
        #[inline(always)]
        unsafe fn w8(base: u32, off: u32, val: u8) {
            unsafe { ((base + off) as *mut u8).write(val) }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_INIT, u32, this);
        w8(this, FLAGS, w8_read(this, FLAGS) & 0xFE);
        for off in [
            0xB4u32, 0xB8, 0xBC, 0x128, 0x12C, 0x130, 0x74, 0xA0, 0xA4, 0xA8, 0xAC, 0x124,
            0x94, 0x7C, 0x90, 0x60, 0x64, 0x68, 0x6C, 0xE8,
        ] {
            w32(this, off, 0);
        }
        w16(this, 0xF0, 0);
        w32(this, 0x80, NONE32);
        w32(this, 0x84, NONE32);
        w32(this, 0x98, NONE16 as u32);
        w16(this, 0xB0, NONE16);
        w8(this, 0xB2, 0);
        w32(this, 0x70, NONE32);
        w32(this, 0x78, NONE32);
        w32(this, 0xEC, TUNING_PACK);
        w32(this, 0xF4, WALK_BLEND);
        w32(this, 0xF8, RUN_BLEND);
        w32(this, 0xFC, DRAW_DIST);
        w32(this, 0x120, 2);
        w8(this, FLAGS, w8_read(this, FLAGS) & 0xFD);
        w16(this, 0x15A, NONE16);
        w16(this, 0x15C, NONE16);
        NONE32
    }
});

#[inline(always)]
unsafe fn w8_read(base: u32, off: u32) -> u8 {
    unsafe { ((base + off) as *const u8).read() }
}
