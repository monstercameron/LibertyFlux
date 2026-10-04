// original: 0x00ba2520 SET_PED_NON_CREATION_AREA
/// Script native `SET_PED_NON_CREATION_AREA` (hash 0x3DAB7D72).
///
/// Forwards six float script arguments (two box corners) to the engine; the original builds them as two stack vec3 structs, which is exactly an in-order six-word call.
export!(cdecl, rw_00ba2520(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5))
    }
});
