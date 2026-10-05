// original: 0x009BA390 CCamScriptInstruction_SetHintMoveInDist::vf2
/// Store the move-in distance operand into the hint camera object.
///
/// Looks up the camera (`callee 1`) and copies the word at `this+0x08` to
/// camera `+0x1A4`. No null check on the lookup.
lf_checker_rt::export!(thiscall, rw_009BA390(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const MOVE_IN_DIST: u32 = 0x1A4;
        const LOOKUP: u32 = 1;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let v = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
        (cam.wrapping_add(MOVE_IN_DIST) as *mut u32).write_unaligned(v);
        0
    }
});
