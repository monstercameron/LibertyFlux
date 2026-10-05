// original: 0x009BA8C0 CCamScriptInstruction_SetPosTargetOffset::vf2
/// Store a four-word position-target offset into a camera.
///
/// Resolves the camera by id (`this+0x08`); when it exists, copies the words
/// at `this+0x10..0x1C` bit for bit to camera `+0x1F0..0x1FC`.
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA8C0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const SRC: u32 = 0x10;
        const DST: u32 = 0x1F0;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            for i in 0..4u32 {
                let w = (this.wrapping_add(SRC + i * 4) as *const u32).read_unaligned();
                (cam.wrapping_add(DST + i * 4) as *mut u32).write_unaligned(w);
            }
        }
        0
    }
});
