// original: 0x00b872b0 SET_CAM_BEHIND_PED
/// Script native `SET_CAM_BEHIND_PED` (hash 0x48740598).
///
/// Forwards one script argument (a character handle) to the engine. No return slot is written.
export!(cdecl, rw_00b872b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
