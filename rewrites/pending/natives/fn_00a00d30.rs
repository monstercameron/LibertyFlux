// original: 0x00a00d30 GET_OBJECT_COORDINATES
/// Script native `GET_OBJECT_COORDINATES` (hash 0x49DA4F9E).
///
/// Forwards 4 script argument(s) to the engine: 4 integer(s).
/// No return slot is written.
export!(cdecl, rw_00a00d30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
