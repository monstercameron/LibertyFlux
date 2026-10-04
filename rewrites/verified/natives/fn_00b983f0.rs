// original: 0x00b983f0 GET_TEXT_INPUT_ACTIVE
/// Script native `GET_TEXT_INPUT_ACTIVE` (hash 0x32A3647C).
///
/// Calls the engine text-input query with no arguments and stores the low
/// byte of the answer (a boolean) into the return slot.
export!(cdecl, rw_00b983f0(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        slot as u32
    }
});
