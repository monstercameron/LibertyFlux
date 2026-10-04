// original: 0x00ba1a90 SET_CHAR_SPHERE_DEFENSIVE_AREA
/// Script native `SET_CHAR_SPHERE_DEFENSIVE_AREA` (hash 0x56AD2409).
///
/// Sets a character's spherical defensive area: reads a character handle
/// plus four float bit-patterns (centre and radius) and issues one
/// 8-word engine call carrying the handle, the four words, then the
/// middle three float words repeated. The original builds the argument
/// block in its own stack frame; only the copied values cross to the
/// callee, so the rewrite passes the same eight words directly.
/// No return slot is written.
export!(cdecl, rw_00ba1a90(ctx: *const u8) -> u32 {
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
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
