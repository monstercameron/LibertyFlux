// original: 0x009BA7F0 CCamScriptInstruction_SetNear::vf2
/// Store the near-plane operand into a camera.
///
/// Resolves the camera by id (`this+0x08`); when it exists, copies the word
/// at `this+0x0C` to camera `+0x64`. Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA7F0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const NEAR: u32 = 0x64;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let v = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            (cam.wrapping_add(NEAR) as *mut u32).write_unaligned(v);
        }
        0
    }
});
