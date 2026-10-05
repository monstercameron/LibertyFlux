// original: 0x009BA360 CCamScriptInstruction_SetHintFOV::vf2
/// Forward the field-of-view operand to the hint camera's FOV setter.
///
/// Looks up the camera (`callee 1`), then calls `callee 2` (thiscall/1) with
/// the float at `this+0x08` as raw bits. No null check on the lookup.
lf_checker_rt::export!(thiscall, rw_009BA360(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const LOOKUP: u32 = 1;
        const SET_FOV: u32 = 2;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let bits = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(SET_FOV, u32, cam, bits);
        0
    }
});
