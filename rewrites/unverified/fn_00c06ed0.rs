// original: 0x00c06ed0 stream_slots_adopt
/// Adopt `src`'s slot array into `this`, reallocating on count change.
///
/// When the slot counts at +4 differ, copies `src`'s count into +4 and +6 and
/// either clears `this`+0 (zero count) or allocates a fresh array through the
/// array helper (stdcall/1). Then rebuilds every slot through the rebuild
/// helper (thiscall/1: destination slot, source slot) and returns `count - 1`
/// (0 for an empty array). Thiscall: one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c06ed0(this: u32, src: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const ARR_FN: u32 = 1;
        const REBUILD: u32 = 2;
        const COUNT_OFF: u32 = 4;
        const CAP_OFF: u32 = 6;
        const SLOT_STRIDE: u32 = 80;
        let want = rd16(src + COUNT_OFF);
        if rd16(this + COUNT_OFF) != want {
            wr16(this + COUNT_OFF, want as u16);
            wr16(this + CAP_OFF, want as u16);
            if want == 0 {
                wr32(this, 0);
            } else {
                let arr: u32 = lf_checker_rt::callee_stdcall!(ARR_FN, u32, want);
                wr32(this, arr);
            }
        }
        let count = rd16(this + COUNT_OFF);
        let mut ret = 0u32;
        let mut i = 0u32;
        while i < count {
            let dst = rd32(this).wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            let from = rd32(src).wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            let _: u32 = lf_checker_rt::callee_thiscall!(REBUILD, u32, dst, from);
            ret = i;
            i += 1;
        }
        ret
    }
});
