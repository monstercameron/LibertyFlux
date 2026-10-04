// original: 0x00ca2740 face_state_init

/// Initialise a face-state object to rest pose.
///
/// Zeroes the object header, sets the two scale words at `+0x08` / `+0x0c`
/// to 1.0f, raises the present flag at `+0x24`, and loads the shared
/// face-table pointer from its global into `+0x14`. Returns `this`.
///
/// Original: 0x00ca2740 (thiscall, no stack words, returns eax = this).
lf_checker_rt::export!(thiscall, rw_00ca2740(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const ONE_BITS: u32 = 0x3f80_0000;
        const FACE_TABLE_GLOBAL: u32 = 0x0171_bba4;
        wr32(this + 0x00, 0);
        wr32(this + 0x0c, ONE_BITS);
        wr32(this + 0x08, ONE_BITS);
        wr32(this + 0x04, 0);
        wr32(this + 0x1c, 0);
        wr8(this + 0x24, 1);
        wr32(this + 0x14, 0);
        wr32(this + 0x10, 0);
        wr32(this + 0x18, 0);
        wr32(this + 0x14, *lf_checker_rt::global::<u32>(FACE_TABLE_GLOBAL));
        this
    }
});
