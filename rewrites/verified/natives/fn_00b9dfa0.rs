// original: 0x00b9dfa0 ARE_ANY_CHARS_NEAR_CHAR
/// Script native `ARE_ANY_CHARS_NEAR_CHAR` (hash 0x0F4A4FB2).
///
/// Forwards two script arguments (a character handle and a radius float, forwarded as bits) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9dfa0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
