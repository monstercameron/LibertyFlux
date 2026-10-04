// original: 0x00ca22d0 slot_ptr_field28

/// Resolve the event-source handle to its payload field.
///
/// Reads the handle from slot `+0x15c`; a null slot resolves to null without
/// calling. Otherwise the handle goes through the lookup helper (callee 1)
/// and the word at `+0x28` of the answer is returned, or null when the
/// lookup itself answers null.
///
/// Original: 0x00ca22d0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca22d0(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe {
        const SLOT: u32 = 0x15c;
        const FIELD: u32 = 0x28;
        let h = rd32(this + SLOT);
        if h == 0 {
            return 0;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(1, u32, h);
        if p == 0 {
            0
        } else {
            rd32(p + FIELD)
        }
    }
});
