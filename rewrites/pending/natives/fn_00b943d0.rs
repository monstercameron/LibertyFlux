// original: 0x00b943d0 GET_CURRENT_WEATHER
/// Script native `GET_CURRENT_WEATHER` (hash 0x27E421EA).
///
/// Forwards one script argument (an out-pointer) to the engine.
/// No return slot is written by the handler itself.
export!(cdecl, rw_00b943d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
