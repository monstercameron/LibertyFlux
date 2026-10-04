// original: 0x00bd8bf0 NETWORK_SEND_TEXT_CHAT
/// Script native `NETWORK_SEND_TEXT_CHAT` (hash 0x18C67E6D).
///
/// Forwards 2 script arguments (a player index and a text pointer) to the engine.
///
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
///
export!(cdecl, rw_00bd8bf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
