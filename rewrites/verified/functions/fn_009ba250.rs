// original: 0x009BA250 CCamScriptInstruction_SetGameCamPitch::vf2
/// Apply the instruction's pitch operand to the game camera, if one exists.
///
/// Identical to the heading setter except the forwarded axis constant is 1.
/// Null lookup: no call, no write.
lf_checker_rt::export!(thiscall, rw_009BA250(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const LOOKUP: u32 = 1;
        const APPLY: u32 = 2;
        const AXIS_PITCH: u32 = 1;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        if cam != 0 {
            let bits = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(APPLY, u32, cam, bits, AXIS_PITCH);
        }
        0
    }
});
