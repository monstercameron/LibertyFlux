// original: 0x009BA590 CCamScriptInstruction_SetLookDampingParams::vf2
/// Forward three damping floats to a camera's damping-params setter.
///
/// Resolves the camera by id (`this+0x08`); when it exists, calls `callee 2`
/// (thiscall/3) with the floats at `this+0x0C..0x14` as raw bits.
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA590(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const LOOKUP: u32 = 1;
        const SET_DAMPING: u32 = 2;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let d0 = (this.wrapping_add(0x0C) as *const u32).read_unaligned();
            let d1 = (this.wrapping_add(0x10) as *const u32).read_unaligned();
            let d2 = (this.wrapping_add(0x14) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_DAMPING, u32, cam, d0, d1, d2);
        }
        0
    }
});
