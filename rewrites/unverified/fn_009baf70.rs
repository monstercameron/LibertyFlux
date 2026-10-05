// original: 0x009BAF70 CCamScriptInstruction_SetSplineSpeedConstant::vf2
/// Store the constant-speed flag operand into a camera.
///
/// Resolves the camera by id (`this+0x08`); when it exists, copies the byte
/// at `this+0x0C` to camera `+0x150`. Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BAF70(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const SPEED_CONST: u32 = 0x150;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let b = (this.wrapping_add(OPERAND) as *const u8).read();
            (cam.wrapping_add(SPEED_CONST) as *mut u8).write(b);
        }
        0
    }
});
