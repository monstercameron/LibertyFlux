// original: 0x00CC71F0 euphoria_set_param_by_key (proposed)

/// Push a parameter word into every enumerated object keyed by `key`.
///
/// Walks every object the enumerator callees yield. Each object whose id word
/// (`+0x10`) equals `key` receives `bits` through the setter callee. Returns
/// nothing meaningful. `bits` is carried opaquely (the original moves it
/// through a vector register onto the stack); only its bit pattern matters.
///
/// Original: 0x00CC71F0 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00cc71f0(obj: u32, key: u32, bits: u32) -> u32 {
    unsafe {
        const FIRST_CALLEE: u32 = 1;
        const SET_CALLEE: u32 = 2;
        const NEXT_CALLEE: u32 = 3;
        const ID_AT: u32 = 0x10;
        let mut cand = lf_checker_rt::callee_thiscall!(FIRST_CALLEE, u32, obj, 0, 2);
        if cand == 0 {
            return 0;
        }
        loop {
            if (cand.wrapping_add(ID_AT) as *const u32).read_unaligned() == key {
                lf_checker_rt::callee_thiscall!(SET_CALLEE, u32, cand, bits);
            }
            cand = lf_checker_rt::callee_thiscall!(NEXT_CALLEE, u32, obj, 0, 2);
            if cand == 0 {
                return 0;
            }
        }
    }
});
