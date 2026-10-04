// original: 0x00ba1260 SET_CHAR_COORDINATES_DONT_WARP_GANG_NO_OFFSET
/// Script native `SET_CHAR_COORDINATES_DONT_WARP_GANG_NO_OFFSET` (hash 0x355F3FEB).
///
/// Forwards four script arguments to the engine: a character handle and
/// three float bit-patterns (coordinates).
/// No return slot is written.
export!(cdecl, rw_00ba1260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        );
        answer
    }
});
