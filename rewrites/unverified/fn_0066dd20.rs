// original: 0x0066DD20 sub_0066dd20

/// Copy a sixteen-word block from a source buffer into this object.
///
/// `this` points to the destination object and `src` to sixteen readable
/// dwords. Words are copied in order from `src[0..16]` to `this[0x00..0x40]`.
/// The value returned is the last word copied (`src[15]`), left in the
/// return register by the final load.
///
/// Original: 0x0066DD20 (thiscall, one stack argument, no calls).
lf_checker_rt::export!(thiscall, rw_0066dd20(this: u32, src: u32) -> u32 {
    unsafe {
        const WORDS: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut last = 0;
        let mut i = 0;
        while i < WORDS {
            last = rd32(src + i * 4);
            wr32(this + i * 4, last);
            i += 1;
        }
        last
    }
});
