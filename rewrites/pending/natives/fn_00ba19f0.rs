// original: 0x00ba19f0 SET_CHAR_RELATIONSHIP_GROUP
/// Script native `SET_CHAR_RELATIONSHIP_GROUP` (hash 0x61822A3C).
///
/// Forwards two script arguments (a character handle and a relationship
/// group) to the engine. No return slot is written.
export!(cdecl, rw_00ba19f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
