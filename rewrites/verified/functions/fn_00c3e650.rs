// original: 0x00c3e650 train_copy_state_9_or_12 (proposed)
/// Copy a sparse state record from `src` into this object.
///
/// `this` (ECX) is the destination, `src` the source record, `flag` selects
/// the width: when its low byte is non-zero the nine dwords at offsets
/// 0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28 are copied and the
/// words at 0x0c and 0x1c are left alone; when it is zero the same nine
/// plus the three at 0x30, 0x34, 0x38 are copied (the 0x0c/0x1c/0x2c words
/// are never touched). Returns the last copied word. No calls.
///
/// Original: 0x00c3e650 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c3e650(this: u32, src: u32, flag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn cp(dst: u32, src: u32, off: u32) -> u32 {
            unsafe {
                let v = ((src + off) as *const u32).read_unaligned();
                ((dst + off) as *mut u32).write_unaligned(v);
                v
            }
        }
        let mut last = cp(this, src, 0x00);
        last = cp(this, src, 0x04);
        last = cp(this, src, 0x08);
        last = cp(this, src, 0x10);
        last = cp(this, src, 0x14);
        last = cp(this, src, 0x18);
        last = cp(this, src, 0x20);
        last = cp(this, src, 0x24);
        last = cp(this, src, 0x28);
        if (flag & 0xff) == 0 {
            last = cp(this, src, 0x30);
            last = cp(this, src, 0x34);
            last = cp(this, src, 0x38);
        }
        last
    }
});
