// original: 0x00b946d0 GET_OVERRIDE_NO_SPRINTING_ON_PHONE_IN_MULTIPLAYER
/// Script native `GET_OVERRIDE_NO_SPRINTING_ON_PHONE_IN_MULTIPLAYER`
/// (hash 0x5B652681).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b946d0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
