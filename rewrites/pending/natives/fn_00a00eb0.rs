// original: 0x00a00eb0 GET_OFFSET_FROM_OBJECT_IN_WORLD_COORDS
/// Script native `GET_OFFSET_FROM_OBJECT_IN_WORLD_COORDS` (hash 0x449F4165).
///
/// Forwards seven script arguments to the engine: an object handle, three
/// float offsets (copied as raw bits, so bit-exact) and three out-pointers.
/// No return slot is written by the handler itself.
export!(cdecl, rw_00a00eb0(ctx: *const u8) -> u32 {
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
            *args.add(5),
            *args.add(6)
        )
    }
});
