// original: 0x00bb64e0 PLAYSTATS_INT_FLOAT
/// Script native `PLAYSTATS_INT_FLOAT` (hash 0x511200C7).
///
/// Forwards three script arguments (two integers and one float bit-pattern)
/// to the engine statistics sender. The float is copied as raw bits, so the
/// forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bb64e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
