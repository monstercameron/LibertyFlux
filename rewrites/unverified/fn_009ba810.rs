// original: 0x009BA810 CCamScriptInstruction_SetNearDOF::vf2
/// Store the near depth-of-field operand into a camera.
///
/// Same shape as the near-plane setter, but the destination is `+0x6C`.
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA810(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const NEAR_DOF: u32 = 0x6C;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let v = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            (cam.wrapping_add(NEAR_DOF) as *mut u32).write_unaligned(v);
        }
        0
    }
});
