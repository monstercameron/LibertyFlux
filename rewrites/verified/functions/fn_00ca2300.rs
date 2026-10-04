// original: 0x00ca2300 slot_ptr_first

/// Resolve the event-source handle to its first word.
///
/// Same shape as `slot_ptr_field28` but returns the word at `+0x00` of the
/// lookup answer.
///
/// Original: 0x00ca2300 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca2300(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe {
        const SLOT: u32 = 0x15c;
        let h = rd32(this + SLOT);
        if h == 0 {
            return 0;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(1, u32, h);
        if p == 0 {
            0
        } else {
            rd32(p)
        }
    }
});
