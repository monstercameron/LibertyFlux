// original: 0x00b2b2a0 obj_record_refresh

/// Refreshes an object's record unless its cached pair is still valid.
///
/// `obj` points to a large object. When the clock minus the object's stamp
/// (compared unsigned) is below 1000, both cached halves (low words at
/// +0xDE0/+0xDE4) differ from 0xFFFF, the first half's table entry is set
/// and the second half's table entry is also set, returns the first half
/// unchanged. Otherwise releases and clears the record, calls the rebuild
/// callee (object parts plus a flag for state word 2), and when the rebuilt
/// first half is not 0xFFFF calls the commit callee and returns its answer;
/// when it is 0xFFFF returns 0xFFFF. Cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_00b2b2a0(obj: u32) -> u32 {
    unsafe {
        const CLOCK: u32 = 0x011735B4;
        const VALID_TAB: u32 = 0x01178284;
        const REBUILD_OBJ: u32 = 0x01177A80;
        const FRESH: u32 = 1000;
        const NONE: u32 = 0xFFFF;
        const RELEASE: u32 = 0;
        const CLEAR: u32 = 1;
        const REBUILD: u32 = 2;
        const COMMIT: u32 = 3;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let clock = rd32(lf_checker_rt::relocated(CLOCK));
        let age = clock.wrapping_sub(rd32(obj + 0xEE8));
        if age < FRESH {
            let a = rd32(obj + 0xDE0) & NONE;
            if a != NONE {
                let c = rd32(obj + 0xDE4) & NONE;
                if c != NONE {
                    let tab = lf_checker_rt::relocated(VALID_TAB);
                    if rd32(tab + a.wrapping_mul(4)) != 0
                        && rd32(tab + c.wrapping_mul(4)) != 0
                    {
                        return a;
                    }
                }
            }
        }
        lf_checker_rt::callee_thiscall!(RELEASE, u32, obj + 0xDD4);
        lf_checker_rt::callee_thiscall!(CLEAR, u32, obj + 0xDD4);
        let flag = if rd32(obj + 0x1304) == 2 { 1u32 } else { 0u32 };
        let inner = rd32(obj + 0x20);
        lf_checker_rt::callee_thiscall!(
            REBUILD, u32, lf_checker_rt::relocated(REBUILD_OBJ),
            inner + 0x30, inner + 0x10, obj + 0xDE0, obj + 0xDE4, flag,
            obj + 0xE12, obj + 0xE2C
        );
        let ax = ((obj + 0xDE0) as *const u16).read_unaligned() as u32;
        if ax == NONE {
            return NONE;
        }
        lf_checker_rt::callee_cdecl!(COMMIT, u32, obj, 5, 0)
    }
});
