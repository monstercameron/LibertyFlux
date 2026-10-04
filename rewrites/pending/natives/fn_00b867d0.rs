// original: 0x00b867d0 CAM_SEQUENCE_REMOVE
/// Script native `CAM_SEQUENCE_REMOVE` (hash 0x01473ACB).
///
/// Forwards one script argument (a camera handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b867d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

