// original: 0x00b93f20 CAN_START_MISSION_PASSED_TUNE
/// Script native `CAN_START_MISSION_PASSED_TUNE` (hash 0x22AB641D).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b93f20(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
