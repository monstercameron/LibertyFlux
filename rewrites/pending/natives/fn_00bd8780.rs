// original: 0x00bd8780 NETWORK_IS_FIND_RESULT_VALID
/// Script native `NETWORK_IS_FIND_RESULT_VALID` (hash 0x51DF00D8).
///
/// Forwards one integer argument to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8780(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
