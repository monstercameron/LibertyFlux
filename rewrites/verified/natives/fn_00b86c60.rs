// original: 0x00b86c60 GET_GAME_CAM_CHILD
/// Script native `GET_GAME_CAM_CHILD` (hash 0x588F6BC0).
///
/// Forwards one script argument to the engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b86c60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
