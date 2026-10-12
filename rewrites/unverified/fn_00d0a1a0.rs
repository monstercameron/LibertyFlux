// original: 0x00D0A1A0 CTaskComplexNewUseCover::vf19

/// Refresh a new-use-cover task's sub-task and cover state.
///
/// Unless the keep bit (bit 2 of byte `+0x35`) is set, detaches the current
/// sub-task for the incoming word (patched stdcall callee), then always
/// rebuilds the cover plan (patched thiscall callee) and re-applies it
/// (patched thiscall callee). Returns the rebuild answer.
///
/// Original: 0x00D0A1A0 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d0a1a0(this: u32, a1: u32) -> u32 {
    unsafe {
        const DETACH: u32 = 1;
        const REBUILD: u32 = 2;
        const APPLY: u32 = 3;
        const KEEP_BIT: u8 = 4;
        const CONTROL: u32 = 0x35;

        if ((this + CONTROL) as *const u8).read() & KEEP_BIT == 0 {
            lf_checker_rt::callee_stdcall!(DETACH, u32, a1, 1);
        }
        let plan = lf_checker_rt::callee_thiscall!(REBUILD, u32, this, a1);
        lf_checker_rt::callee_thiscall!(APPLY, u32, this, a1);
        plan
    }
});
