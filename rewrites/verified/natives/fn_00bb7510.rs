// original: 0x00bb7510 REMOVE_IPL
/// Script native `REMOVE_IPL` (hash 0x787F38B5).
///
/// Forwards one map-item name hash to the engine's streaming worker. No
/// return slot is written.
export!(cdecl, rw_00bb7510(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
