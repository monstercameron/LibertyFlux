// original: 0x00bd79b0 GET_GFWL_HAS_SAFE_HOUSE
/// Script native `GET_GFWL_HAS_SAFE_HOUSE` (hash 0x6CC85D46).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd79b0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
