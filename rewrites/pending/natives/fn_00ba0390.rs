// original: 0x00ba0390 LOCATE_CHAR_ANY_MEANS_3D
/// Script native `LOCATE_CHAR_ANY_MEANS_3D` (hash 0x0437222B).
///
/// Reports whether a character is within a 3D box by any means. Forwards
/// eight script arguments to the engine: the character handle, six float
/// bit-patterns (box coordinates, copied as raw bits) and a boolean flag
/// coerced with `arg != 0`. Stores the low byte of the engine answer
/// (zero-extended) into the return slot.
///
/// Quirk (observed): as in `ALLOW_REACTION_ANIMS`, the flag is coerced into
/// the low byte of the handler's own incoming stack slot, so the pushed
/// dword's high bytes repeat the context pointer; reproduced here exactly.
export!(cdecl, rw_00ba0390(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(7) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            quirked,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
