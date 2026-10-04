// original: 0x00b9f3a0 GET_NUMBER_OF_CHAR_DRAWABLE_VARIATIONS
/// Script native `GET_NUMBER_OF_CHAR_DRAWABLE_VARIATIONS` (hash 0x3C293296).
///
/// Forwards two script arguments (a character handle and a variation slot)
/// to the engine and stores its full 32-bit answer into the return slot.
/// Unlike the boolean natives, this handler keeps the whole answer
/// (`mov`, not `movzx`).
export!(cdecl, rw_00b9f3a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
