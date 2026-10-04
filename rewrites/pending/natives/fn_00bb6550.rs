// original: 0x00bb6550 PLAYSTATS_MISSION_STARTED
/// Script native `PLAYSTATS_MISSION_STARTED` (hash 0x26747EBE).
///
/// Forwards one script argument (a mission id) to the engine statistics
/// reporter. No return slot is written.
export!(cdecl, rw_00bb6550(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
