// original: 0x00b941b0 DID_SAVE_COMPLETE_SUCCESSFULLY
/// Script native `DID_SAVE_COMPLETE_SUCCESSFULLY` (hash 0x5AA33E86).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b941b0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
