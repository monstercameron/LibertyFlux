// original: 0x009BA7B0 CCamScriptInstruction_SetMotionBlur::vf2
/// Clamp the motion-blur operand into [0, max] and store it on a camera.
///
/// Resolves the camera by id (`this+0x08`); when it exists, clamps the float
/// at `this+0x0C` below by 0.0 and above by the read-only maximum, then
/// stores the result at camera `+0x74`. The comparisons are ordered-float
/// `above` checks, so a NaN operand passes through unchanged; only plain
/// bit copies otherwise, no arithmetic, so float order needs no pinning.
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA7B0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const BLUR: u32 = 0x74;
        const BLUR_MAX: u32 = 0xFE88E8;
        const LOOKUP: u32 = 1;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let x = f32::from_bits((this.wrapping_add(OPERAND) as *const u32).read_unaligned());
            let max = f32::from_bits(lf_checker_rt::global::<u32>(BLUR_MAX).read_unaligned());
            let r = if x < 0.0 {
                0.0f32
            } else if x > max {
                max
            } else {
                x
            };
            (cam.wrapping_add(BLUR) as *mut u32).write_unaligned(r.to_bits());
        }
        0
    }
});
