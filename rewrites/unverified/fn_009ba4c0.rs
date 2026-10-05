// original: 0x009BA4C0 CCamScriptInstruction_SetInterpStateSrc::vf2
/// Store the interpolation source-state operand into a camera.
///
/// Resolves the camera by id (`callee 1` with the word at `this+0x08`) and,
/// when it exists, copies the word at `this+0x0C` to camera `+0x15C`.
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA4C0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const SRC: u32 = 0x0C;
        const STATE_SRC: u32 = 0x15C;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let v = (this.wrapping_add(SRC) as *const u32).read_unaligned();
            (cam.wrapping_add(STATE_SRC) as *mut u32).write_unaligned(v);
        }
        0
    }
});
