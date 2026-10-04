// original: 0x00b8ccb0 IS_HELP_MESSAGE_BEING_DISPLAYED
/// Script native `IS_HELP_MESSAGE_BEING_DISPLAYED` (hash 0x6E4E1BEC).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the return
/// slot. Returns the slot pointer.
export!(cdecl, rw_00b8ccb0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
