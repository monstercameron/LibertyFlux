// original: 0x00bd7880 DOES_PLAYER_HAVE_CONTROL_OF_NETWORK_ID
/// Script native `DOES_PLAYER_HAVE_CONTROL_OF_NETWORK_ID` (hash 0x3D0B5E56).
///
/// Forwards a player index and a network id to the engine, then stores the
/// low byte of the engine answer (zero-extended) into the return slot.
/// Returns the slot pointer, as the original leaves it in EAX.
export!(cdecl, rw_00bd7880(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
