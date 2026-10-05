// original: 0x009BA400 CCamScriptInstruction_SetHintTimesDefault::vf2
/// Tail-call the hint camera's reset-times method.
///
/// Looks up the camera (`callee 1`) and jumps to `callee 2` (thiscall/0)
/// with it, forwarding the result. Same shape as the move-in-distance reset.
lf_checker_rt::export!(thiscall, rw_009BA400(_this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const LOOKUP: u32 = 1;
        const RESET: u32 = 2;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        lf_checker_rt::callee_thiscall!(RESET, u32, cam)
    }
});
