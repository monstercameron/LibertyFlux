// original: 0x00bd1190 SET_CHAR_AMMO
/// Script native `SET_CHAR_AMMO` (hash 0x437D247E).
///
/// Forwards three script arguments (a character handle, a weapon number and
/// an ammo count) to the engine. No return slot is written.
export!(cdecl, rw_00bd1190(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
