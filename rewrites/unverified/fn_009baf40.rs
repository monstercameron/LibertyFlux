// original: 0x009BAF40 CCamScriptInstruction_SetSplineProgress::vf2
/// Forward the spline-progress operand to a camera's progress setter.
///
/// Resolves the camera by id (`this+0x08`); when it exists, calls `callee 2`
/// (thiscall/1) with the float at `this+0x0C` as raw bits.
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BAF40(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const LOOKUP: u32 = 1;
        const SET_PROGRESS: u32 = 2;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let bits = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_PROGRESS, u32, cam, bits);
        }
        0
    }
});
