// original: 0x009cc730 SET_VARIABLE_ON_SOUND
/// Script native `SET_VARIABLE_ON_SOUND` (hash 0x39200B83).
///
/// Forwards three script arguments to the engine: a sound handle, a variable
/// id and a float bit-pattern (the value). No return slot is written.
export!(cdecl, rw_009cc730(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
