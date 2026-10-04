// original: 0x00bd9640 SET_SERVER_ID
/// Script native `SET_SERVER_ID` (hash 0x575136AC).
///
/// Forwards one script argument (a server id) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd9640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
