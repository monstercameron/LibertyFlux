// original: 0x00ba2900 STORE_CAR_CHAR_IS_IN_NO_SAVE
/// Script native `STORE_CAR_CHAR_IS_IN_NO_SAVE` (hash 0x21CC647F).
///
/// Forwards two script arguments (a character handle and a vehicle handle)
/// to the engine. No return slot is written.
export!(cdecl, rw_00ba2900(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
