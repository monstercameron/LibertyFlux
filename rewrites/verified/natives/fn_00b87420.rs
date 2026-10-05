// original: 0x00b87420 SET_CAM_INTERP_DETAIL_ROT_STYLE_ANGLES
/// Script native `SET_CAM_INTERP_DETAIL_ROT_STYLE_ANGLES` (hash 0x5F7307F4).
///
/// Forwards one script argument to the engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b87420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
