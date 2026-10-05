// original: 0x009BA420 CCamScriptInstruction_SetInterpDetail_Rot_Style_Angles::vf2
/// Select angle interpolation for a camera's rotation detail.
///
/// Resolves the camera by id (`callee 1`, thiscall/1 with the word at
/// `this+0x08`). When the camera exists, clears the style byte at `+0x1E8`
/// (0 selects angles). A null lookup does nothing.
lf_checker_rt::export!(thiscall, rw_009BA420(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const ROT_STYLE: u32 = 0x1E8;
        const STYLE_ANGLES: u8 = 0;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            (cam.wrapping_add(ROT_STYLE) as *mut u8).write(STYLE_ANGLES);
        }
        0
    }
});
