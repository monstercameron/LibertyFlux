// original: 0x00976fb0 audio_first_result_or_second (proposed)

/// Resolve two optional handles and return the first good result.
///
/// Each of the pointers at +0x20 and +0x24, when non-null, is resolved
/// through the lookup callee. Returns the first result when it is nonzero,
/// else the second (which is 0 when its handle was null). Null checks only.
/// Original: 0x00976FB0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00976fb0(obj: u32) -> u32 {
    unsafe {
        const H1: u32 = 0x20;
        const H2: u32 = 0x24;
        const LOOK1: u32 = 1;
        const LOOK2: u32 = 2;
        let c1 = ((obj.wrapping_add(H1)) as *const u32).read_unaligned();
        let c2 = ((obj.wrapping_add(H2)) as *const u32).read_unaligned();
        let r1 = if c1 != 0 {
            lf_checker_rt::callee_thiscall!(LOOK1, u32, c1)
        } else {
            0
        };
        let r2 = if c2 != 0 {
            lf_checker_rt::callee_thiscall!(LOOK2, u32, c2)
        } else {
            0
        };
        if r1 != 0 { r1 } else { r2 }
    }
});
