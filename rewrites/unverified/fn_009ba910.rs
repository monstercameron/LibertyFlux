// original: 0x009BA910 CCamScriptInstruction_SetPosTargetOffsetRelative::vf2
/// Set or clear the offset-relative flag (bit 0) on a camera.
///
/// Resolves the camera by id (`this+0x08`); when it exists, writes bit 0 of
/// the byte at `this+0x0C` into bit 0 of the flag byte at camera `+0x264`,
/// preserving the other bits. Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA910(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const FLAGS: u32 = 0x264;
        const RELATIVE: u8 = 0x01;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let b = (this.wrapping_add(OPERAND) as *const u8).read();
            let fs = cam.wrapping_add(FLAGS) as *mut u8;
            let old = fs.read();
            fs.write(if b & 1 != 0 { old | RELATIVE } else { old & !RELATIVE });
        }
        0
    }
});
