// original: 0x006932E0 track_heap_sort_pass (proposed)

/// One extraction pass of a heapsort over the array at `base`: `end` points
/// one past the last live word, so (end-base) is the live length in bytes.
/// While the length masked down to a word count is strictly above 4
/// (signed), the last live word is lifted out, the first word takes its
/// place, and the intercepted sift-down helper is called with
/// (base, 0, (len-4)/4, lifted, extra); the length shrinks by one word per
/// pass. No return value.
///
/// Original: 0x006932E0 (ECX+EDX plus one stack argument with caller
/// cleanup, so the esp check is off for this function).
lf_checker_rt::export!(fastcall, rw_006932E0(base: u32, end: u32, extra: u32) -> u32 {
    unsafe {
        const SIFT: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut len = end.wrapping_sub(base);
        if ((len & 0xFFFF_FFFC) as i32) > 4 {
            loop {
                let last = rd32(base.wrapping_add(len).wrapping_sub(4));
                let first = rd32(base);
                let newlen = len.wrapping_sub(4);
                wr32(base.wrapping_add(len).wrapping_sub(4), first);
                let idx = ((newlen as i32) >> 2) as u32;
                lf_checker_rt::callee_fastcall!(SIFT, u32, base, 0, idx, last, extra);
                len = newlen;
                if ((len & 0xFFFF_FFFC) as i32) <= 4 {
                    break;
                }
            }
        }
        0
    }
});
