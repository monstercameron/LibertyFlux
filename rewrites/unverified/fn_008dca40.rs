// original: 0x008DCA40 counted_array_alloc_zero (proposed)

/// Counted array allocation with zeroing (proposed name).
///
/// Builds a zeroed word array of `count` entries. The count comes
/// from the low 16 bits of the first callee's answer for `key` and is
/// stored at +0x04; the allocator callee returns the block, stored at
/// +0x00, and every entry is zeroed. The low byte of `tag` is stored at
/// +0x0b and the count is returned. The `count * 4` multiply cannot
/// overflow (count is 16 bits), so the overflow path is dead. The list
/// size (88) is 4 short; the real body is 92 bytes.
///
/// Original: 0x008DCA40 (thiscall: `this` in ECX, `key`, `tag`).
lf_checker_rt::export!(thiscall, rw_008dca40(this: u32, key: u32, tag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        const BLOCK: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const TAG: u32 = 0x0b;
        let count = lf_checker_rt::callee_cdecl!(1, u32, key) & 0xffff;
        (this.wrapping_add(COUNT) as *mut u16).write_unaligned(count as u16);
        let block = lf_checker_rt::callee_cdecl!(2, u32, count.wrapping_mul(4));
        (this.wrapping_add(BLOCK) as *mut u32).write_unaligned(block);
        let mut i: u32 = 0;
        while i < count {
            ((block.wrapping_add(i.wrapping_mul(4))) as *mut u32).write_unaligned(0);
            i += 1;
        }
        (this.wrapping_add(TAG) as *mut u8).write(tag as u8);
        count
    }
});
