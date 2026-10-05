// original: 0x00b86c40 GET_FREE_CAM
/// Script native `GET_FREE_CAM` (hash 0x538514CC).
///
/// Forwards one script argument (an out-pointer for the camera handle) to
/// the engine. No return slot is written by the handler itself.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b86c40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
