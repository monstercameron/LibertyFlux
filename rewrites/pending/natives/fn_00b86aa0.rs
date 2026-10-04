// original: 0x00b86aa0 END_CAM_COMMANDS
/// Script native `END_CAM_COMMANDS` (hash 0x627F3275).
///
/// Forwards one script argument (a camera handle) to the engine to end its
/// command block. No return slot is written; the engine answer (a status
/// byte) is the exit value.
export!(cdecl, rw_00b86aa0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
