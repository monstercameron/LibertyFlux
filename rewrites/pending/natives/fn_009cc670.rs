// original: 0x009cc670 SET_SCRIPT_MIC_POSITION
/// Script native `SET_SCRIPT_MIC_POSITION` (hash 0x295D3A87).
///
/// Forwards three float bit-patterns (coordinates) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc670(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
