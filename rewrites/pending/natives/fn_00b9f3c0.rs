// original: 0x00b9f3c0 GET_NUMBER_OF_CHAR_TEXTURE_VARIATIONS
/// Script native `GET_NUMBER_OF_CHAR_TEXTURE_VARIATIONS` (hash 0x06C4113E).
///
/// Forwards three script arguments (a character handle and two texture
/// indices) to the engine and stores its full 32-bit answer into the
/// return slot.
export!(cdecl, rw_00b9f3c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
