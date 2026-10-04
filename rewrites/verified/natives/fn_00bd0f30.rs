// original: 0x00bd0f30 GET_MAX_AMMO_IN_CLIP
/// Script native `GET_MAX_AMMO_IN_CLIP` (hash 0x01794A3C).
///
/// Forwards three script arguments (a character handle, a weapon slot and
/// an output slot) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00bd0f30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
