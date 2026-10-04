// original: 0x00bb1eb0 GET_PLAYERSETTINGS_MODEL_CHOICE
/// Script native `GET_PLAYERSETTINGS_MODEL_CHOICE` (hash 0x116E5A1F).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores all 32 bits of its answer into the return slot.
export!(cdecl, rw_00bb1eb0(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
