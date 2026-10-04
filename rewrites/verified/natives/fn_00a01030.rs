// original: 0x00a01030 GRAB_ENTITY_ON_ROPE_FOR_OBJECT
/// Script native `GRAB_ENTITY_ON_ROPE_FOR_OBJECT` (hash 0x309F1F4B).
///
/// Forwards 4 script arguments (four object/entity handles) to the engine.
///
/// No return slot is written.
///
export!(cdecl, rw_00a01030(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
