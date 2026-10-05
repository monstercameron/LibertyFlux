// original: 0x009BA440 CCamScriptInstruction_SetInterpDetail_Rot_Style_Quats::vf2
/// Select quaternion interpolation for a camera's rotation detail.
///
/// Same shape as the angles variant, but stores 1 at camera `+0x1E8`.
/// A null lookup does nothing.
lf_checker_rt::export!(thiscall, rw_009BA440(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const ROT_STYLE: u32 = 0x1E8;
        const STYLE_QUATS: u8 = 1;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            (cam.wrapping_add(ROT_STYLE) as *mut u8).write(STYLE_QUATS);
        }
        0
    }
});
