// original: 0x009BA220 CCamScriptInstruction_SetGameCamHeading::vf2
/// Apply the instruction's heading operand to the game camera, if one exists.
///
/// Looks up the camera through the hint manager (`callee 1`, thiscall/0) and,
/// when the lookup returns non-null, forwards (camera, operand bits, axis 0)
/// to the cdecl/3 applier (`callee 2`). The operand is the float at
/// `this+0x08`, passed as raw bits. A null lookup does nothing.
lf_checker_rt::export!(thiscall, rw_009BA220(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const LOOKUP: u32 = 1;
        const APPLY: u32 = 2;
        const AXIS_HEADING: u32 = 0;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        if cam != 0 {
            let bits = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(APPLY, u32, cam, bits, AXIS_HEADING);
        }
        0
    }
});
