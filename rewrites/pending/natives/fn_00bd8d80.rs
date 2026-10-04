// original: 0x00bd8d80 NETWORK_SET_TALKER_PROXIMITY
/// Script native `NETWORK_SET_TALKER_PROXIMITY` (hash 0x2F542797).
///
/// Forwards one float bit-pattern to the engine. No return slot is written.
export!(cdecl, rw_00bd8d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
