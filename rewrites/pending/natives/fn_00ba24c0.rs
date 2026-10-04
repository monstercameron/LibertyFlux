// original: 0x00ba24c0 SET_PED_HELMET_TEXTURE_INDEX
/// Script native `SET_PED_HELMET_TEXTURE_INDEX` (hash 0x6AC14091).
///
/// Forwards two script arguments (a character handle and a texture index)
/// to the engine. No return slot is written.
export!(cdecl, rw_00ba24c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
