// original: 0x00b94d00 SET_BIT
/// Script native `SET_BIT` (hash 0x39551B76).
///
/// Forwards two script arguments (a value and a bit index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b94d00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
