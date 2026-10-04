// original: 0x00b942d0 GENERATE_RANDOM_FLOAT
/// Script native `GENERATE_RANDOM_FLOAT` (hash 0x380C142A).
///
/// Forwards one script argument (an out-pointer the engine writes the
/// generated value through) to the engine. No return slot is written by the
/// handler itself.
export!(cdecl, rw_00b942d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
