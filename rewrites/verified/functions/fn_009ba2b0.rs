// original: 0x009BA2B0 CCamScriptInstruction_SetGameFollowVehicleCamSubMode::vf2
/// Set the follow-vehicle camera sub-mode and remember the raw operand.
///
/// Resolves the camera (`callee 1`), then the follow-vehicle sub-object
/// (`callee 2`, thiscall/2 with args (2, 0)). When that is non-null and the
/// signed operand at `this+0x08` is not negative, its low byte is stored at
/// sub `+0x22C`. When the operand is exactly -1, the stored byte (if it is 5
/// or less, unsigned) replaces the sub-mode global; otherwise the global
/// keeps its value. The raw operand is always written to the last-operand
/// global. Edge cases: operand -1 writes nothing to the object but may write
/// the global; other negatives write neither.
lf_checker_rt::export!(thiscall, rw_009BA2B0(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const SUB_MODE: u32 = 0x22C;
        const SUB_MODE_GLOBAL: u32 = 0x103B924;
        const LAST_OPERAND_GLOBAL: u32 = 0x1039298;
        const MAX_SUB_MODE: u8 = 5;
        const LOOKUP: u32 = 1;
        const FOLLOW_CAM: u32 = 2;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let sub = lf_checker_rt::callee_thiscall!(FOLLOW_CAM, u32, cam, 2, 0);
        let operand = (this.wrapping_add(OPERAND) as *const i32).read_unaligned();
        if sub != 0 {
            if operand > -1 {
                (sub.wrapping_add(SUB_MODE) as *mut u8).write(operand as u8);
            }
            if operand == -1 {
                let stored = (sub.wrapping_add(SUB_MODE) as *const u8).read();
                let keep = lf_checker_rt::global::<u8>(SUB_MODE_GLOBAL).read();
                lf_checker_rt::global::<u8>(SUB_MODE_GLOBAL)
                    .write(if stored <= MAX_SUB_MODE { stored } else { keep });
            }
        }
        lf_checker_rt::global::<i32>(LAST_OPERAND_GLOBAL).write_unaligned(operand);
        0
    }
});
