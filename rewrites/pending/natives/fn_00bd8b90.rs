// original: 0x00bd8b90 NETWORK_RETURN_TO_RENDEZVOUS
/// Script native `NETWORK_RETURN_TO_RENDEZVOUS` (hash 0x00031EC6).
///
/// Script arguments: none.
///
/// Forwards no arguments to the engine routine.
/// The engine's byte answer is zero-extended into the return slot.
export!(cdecl, rw_00bd8b90(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const *mut u32);
        let answer: u32 = callee_cdecl!(1, u32, );
        *slot = answer & 0xFF;
        slot as u32
    }
});
