// original: 0x00bd7860 DOES_PED_EXIST_WITH_NETWORK_ID
/// Script native `DOES_PED_EXIST_WITH_NETWORK_ID` (hash 0x21641887).
///
/// Forwards one script argument (a network id) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7860(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
