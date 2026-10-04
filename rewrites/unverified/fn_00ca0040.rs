// original: 0x00ca0040 ik_state_zero

/// Reset an IK timing/state block to its rest values.
///
/// `this` points to a block holding two timer pairs and a 3x3-ish float area.
/// Words at `+0x120`, `+0x124`, `+0x128`, `+0x12c`, `+0x130`, `+0x138`,
/// `+0x13c` are zeroed and the word at `+0x134` is set to -1.0f
/// (`0xbf800000`). No value is returned.
///
/// Original: 0x00ca0040 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca0040(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const NEG_ONE_BITS: u32 = 0xbf80_0000;
        wr32(this + 0x128, 0);
        wr32(this + 0x124, 0);
        wr32(this + 0x120, 0);
        wr32(this + 0x138, 0);
        wr32(this + 0x13c, 0);
        wr32(this + 0x134, NEG_ONE_BITS);
        wr32(this + 0x130, 0);
        wr32(this + 0x12c, 0);
        0
    }
});
