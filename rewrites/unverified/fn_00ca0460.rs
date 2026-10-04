// original: 0x00ca0460 release_ref_44

/// Release the link at `+0x44` through its vtable and clear it.
///
/// Loads the linked object; a null link does nothing. Otherwise slot 0 of
/// the object's vtable is called with the object and 1 (callee 1), then the
/// link is cleared. No value is returned. The indirect call goes through
/// the fabricated object exactly like the original, landing on the same
/// planted stub.
///
/// Original: 0x00ca0460 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca0460(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const LINK: u32 = 0x44;
        let o = rd32(this + LINK);
        if o == 0 {
            return 0;
        }
        let slot = rd32(rd32(o));
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        f(o, 1);
        wr32(this + LINK, 0);
        0
    }
});
