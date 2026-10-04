// original: 0x00b9f260 GET_GROUP_FORMATION_SPACING
/// Script native `GET_GROUP_FORMATION_SPACING`.
///
/// Forwards two script arguments (a group handle and an out-pointer for the
/// spacing value) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00b9f260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
