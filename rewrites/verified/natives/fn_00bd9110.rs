// original: 0x00bd9110 SEND_CLIENT_BROADCAST_VARIABLES_NOW
/// Script native `SEND_CLIENT_BROADCAST_VARIABLES_NOW` (hash 0x36B40989).
///
/// Forwards one script argument to the engine. Writes no return slot.
export!(cdecl, rw_00bd9110(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0))
    }
});
