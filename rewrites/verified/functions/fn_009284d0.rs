// original: 0x009284d0 input_ui_state_copy (proposed)

/// Copy a UI state record field by field, delegating two sub-objects.
///
/// `dst` (in ECX) receives a copy of the 0x26c-byte record at `src`: plain
/// word copies cover `[0x00..0x60)`, `[0xa0..0x1a0)` and the six words at
/// `0x250..0x268` except the padding hole at `0x25c`, while the
/// `[0x60..0xa0)` and `[0x1a0..0x250)` sub-objects are copied by two
/// intercepted callees (thiscall, destination in ECX, source on the stack).
/// Returns `dst`.
///
/// Original: 0x009284d0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009284d0(dst: u32, src: u32) -> u32 {
    unsafe {
        const CALLEE_HEAD: u32 = 1;
        const CALLEE_TAIL: u32 = 2;
        const HEAD_OFF: u32 = 0x60;
        const HEAD_END: u32 = 0xa0;
        const TAIL_OFF: u32 = 0x1a0;
        const TAIL_END: u32 = 0x250;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn copy_words(dst: u32, src: u32, from: u32, to: u32) {
            unsafe {
                let mut off = from;
                while off < to {
                    wr32(dst.wrapping_add(off), rd32(src.wrapping_add(off)));
                    off += 4;
                }
            }
        }

        copy_words(dst, src, 0, HEAD_OFF);
        lf_checker_rt::callee_thiscall!(
            CALLEE_HEAD,
            u32,
            dst.wrapping_add(HEAD_OFF),
            src.wrapping_add(HEAD_OFF)
        );
        copy_words(dst, src, HEAD_END, TAIL_OFF);
        lf_checker_rt::callee_thiscall!(
            CALLEE_TAIL,
            u32,
            dst.wrapping_add(TAIL_OFF),
            src.wrapping_add(TAIL_OFF)
        );
        for off in [0x250u32, 0x254, 0x258, 0x260, 0x264, 0x268] {
            wr32(dst.wrapping_add(off), rd32(src.wrapping_add(off)));
        }
        dst
    }
});
