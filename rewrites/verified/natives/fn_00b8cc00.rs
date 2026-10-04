// original: 0x00b8cc00 HAS_THIS_ADDITIONAL_TEXT_LOADED
/// Script native `HAS_THIS_ADDITIONAL_TEXT_LOADED` (hash 0x6CF248FD).
///
/// Forwards two script arguments (a text slot and flags) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8cc00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
