// original: 0x00b9e210 ATTACH_PED_TO_OBJECT_PHYSICALLY
/// Script native `ATTACH_PED_TO_OBJECT_PHYSICALLY` (hash 0x782E78BF).
///
/// Forwards ten script arguments: four integers (handles and flags), four
/// float bit-patterns (offsets), and two booleans coerced from words.
/// The first boolean's high bytes are uninitialized stack scratch in the
/// original, so the contract defines a zero stack fill and the rewrite
/// passes the clean boolean; the second boolean reuses the incoming stack
/// slot (reproduced exactly from the context pointer). No return slot.
export!(cdecl, rw_00b9e210(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ctxw = ctx as u32;
        let b8 = ((*args.add(8) != 0) as u32);
        let q9 = (ctxw & !0xFF) | ((*args.add(9) != 0) as u32);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), b8, q9)
    }
});
