// original: 0x00b8cdb0 IS_THIS_HELP_MESSAGE_BEING_DISPLAYED
/// Script native `IS_THIS_HELP_MESSAGE_BEING_DISPLAYED` (hash 0x505D37D8).
///
/// Forwards one script argument (a help-message id) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8cdb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
