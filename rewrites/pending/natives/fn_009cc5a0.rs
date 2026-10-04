// original: 0x009cc5a0 SET_MOBILE_RING_TYPE
/// Script native `SET_MOBILE_RING_TYPE` (hash 0x24885050).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_009cc5a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
