// original: 0x00b94590 GET_IS_AUTOSAVE_OFF
/// Script native `GET_IS_AUTOSAVE_OFF` (hash 0x551C6295).
///
/// Takes no script arguments: calls the engine autosave-state query with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b94590(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
