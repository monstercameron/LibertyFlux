// original: 0x00bd7e10 IS_IN_SPECTATOR_MODE
/// Native handler `IS_IN_SPECTATOR_MODE`.
///
/// Report whether the local player is in spectator mode.
/// Forwards 0 argument(s) to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7e10(ctx: *const u32) -> u32 {
    unsafe {
        // The original keeps only the low byte of the answer.
        let answer = callee_cdecl!(1, u32,);
        *(*ctx as *mut u32) = answer & 0xFF;
        0
    }
});
