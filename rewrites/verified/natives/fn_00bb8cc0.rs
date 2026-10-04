// original: 0x00bb8cc0 GET_SOUND_LEVEL_AT_COORDS
/// Script native `GET_SOUND_LEVEL_AT_COORDS` (hash 0x433E74C6).
///
/// Forwards five script arguments to the engine: an id, three float
/// bit-patterns (coordinates) and a flags dword. Floats are copied as raw
/// bits. No return slot is written by the handler itself.
export!(cdecl, rw_00bb8cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
