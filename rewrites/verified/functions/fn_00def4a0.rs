// original: 0x00DEF4A0 RP_CREATENEW

/// On the covered path, query the object's entry child and, when its low
/// byte is nonzero, run the stack-cookie check and return the full query
/// result. The proof drives only this short path; the long UI construction
/// path is outside this staged rewrite and currently faults deliberately.
lf_checker_rt::export!(thiscall, rw_00def4a0(this: u32) -> u32 {
    unsafe {
        const PROBE_SLOT: u32 = 0x140;
        const COOKIE_CALLEE: u32 = 2;

        let vt = (this as *const u32).read_unaligned();
        let target = ((vt + PROBE_SLOT) as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        let r = probe(this);
        if r & 0xFF != 0 {
            // The cookie-check call receives a side-specific stack value.
            lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, 0);
            r
        } else {
            core::ptr::read_volatile(0 as *const u32)
        }
    }
});
