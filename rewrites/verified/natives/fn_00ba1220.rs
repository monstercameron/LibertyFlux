// original: 0x00ba1220 SET_CHAR_COORDINATES_DONT_WARP_GANG
/// Script native `SET_CHAR_COORDINATES_DONT_WARP_GANG` (hash 0x624E5833).
///
/// Forwards four script arguments to the engine: a character handle and
/// three float bit-patterns (coordinates). No return slot is written.
export!(cdecl, rw_00ba1220(ctx: *const u8) -> u32 {
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
