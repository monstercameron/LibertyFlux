// original: 0x00B86980 DESTROY_CAM
// Rewrite of the DESTROY_CAM native handler.

/// Script native `DESTROY_CAM(cam)`.
///
/// Forwards the camera handle to the engine destroy routine. No return slot
/// is written; the engine's answer is left in the return register.
export!(cdecl, rw_b86980(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args)
    }
});
