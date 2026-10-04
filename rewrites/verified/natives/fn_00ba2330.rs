// original: 0x00ba2330 SET_PED_ALPHA
/// Script native `SET_PED_ALPHA` (hash 0x5AA1795C).
///
/// Forwards two script arguments (a character handle and an alpha value) to the engine. No return slot is written.
export!(cdecl, rw_00ba2330(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
