// original: 0x00c9f340 ik_param_store

/// Store two IK tuning floats and mark the block dirty.
///
/// `this` points to the IK parameter block. The first two stack words are
/// float bits stored at `+0x84` and `+0x88`; the remaining two stack words
/// are ignored. Byte `+0xbc` is set to 1 (dirty) and the function returns 1.
///
/// Original: 0x00c9f340 (thiscall, four stack words, returns al = 1).
lf_checker_rt::export!(thiscall, rw_00c9f340(this: u32, a: u32, b: u32, _c: u32, _d: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const PARAM_A: u32 = 0x84;
        const PARAM_B: u32 = 0x88;
        const DIRTY: u32 = 0xbc;
        wr32(this + PARAM_A, a);
        wr32(this + PARAM_B, b);
        wr8(this + DIRTY, 1);
        1
    }
});
