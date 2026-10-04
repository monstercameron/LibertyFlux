// original: 0x00b93de0 ALLOW_ONE_TIME_ONLY_COMMANDS_TO_RUN
/// Script native `ALLOW_ONE_TIME_ONLY_COMMANDS_TO_RUN` (hash 0x3B2E3198).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b93de0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
