// original: 0x00b9f070 GET_CHAR_TEXTURE_VARIATION
/// Script native `GET_CHAR_TEXTURE_VARIATION` (hash 0x3A7B78C5).
///
/// Forwards two script arguments (a character handle and a texture index)
/// to the engine and stores the full 32-bit answer into the return slot.
/// The answer is the exit value.
export!(cdecl, rw_00b9f070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
