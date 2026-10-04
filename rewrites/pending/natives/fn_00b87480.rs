// original: 0x00b87480 SET_CAM_IN_FRONT_OF_PED
/// Script native `SET_CAM_IN_FRONT_OF_PED` (hash 0x423661A7).
///
/// Forwards one script argument (a character handle) to the engine. No return
/// slot is written.
export!(cdecl, rw_00b87480(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
