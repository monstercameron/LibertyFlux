// original: 0x009cbb20 FIND_STATIC_EMITTER_INDEX
/// Script native `FIND_STATIC_EMITTER_INDEX` (hash 0x64793A54).
///
/// Forwards arg0 to the engine.
/// Writes the full engine answer into the return slot.
export!(cdecl, rw_009cbb20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans;
        ans
    }
});
