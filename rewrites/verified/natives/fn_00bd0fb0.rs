// original: 0x00bd0fb0 GET_WEAPONTYPE_MODEL
/// Script native `GET_WEAPONTYPE_MODEL`.
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd0fb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
