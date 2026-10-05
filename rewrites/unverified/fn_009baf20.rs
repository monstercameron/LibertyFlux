// original: 0x009BAF20 CCamScriptInstruction_SetSplineDuration::vf2
/// Store the spline-duration operand into a camera.
///
/// Resolves the camera by id (`this+0x08`); when it exists, copies the word
/// at `this+0x0C` to camera `+0x144`. Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BAF20(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const DURATION: u32 = 0x144;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let v = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            (cam.wrapping_add(DURATION) as *mut u32).write_unaligned(v);
        }
        0
    }
});
