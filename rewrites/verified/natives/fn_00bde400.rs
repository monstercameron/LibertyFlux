// original: 0x00bde400 SYNCH_RECORDING_WITH_WATER
/// Script native `SYNCH_RECORDING_WITH_WATER` (hash 0x018A0EE0).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bde400(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
