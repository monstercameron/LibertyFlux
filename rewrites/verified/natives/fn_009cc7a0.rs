// original: 0x009cc7a0 START_CUSTOM_MOBILE_PHONE_RINGING
/// Script native `START_CUSTOM_MOBILE_PHONE_RINGING` (hash 0x59406EB1).
///
/// Forwards one script argument to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc7a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
