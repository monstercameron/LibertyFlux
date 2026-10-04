// original: 0x00bb8c00 GET_PED_PATH_MAY_USE_CLIMBOVERS
/// Script native `GET_PED_PATH_MAY_USE_CLIMBOVERS` (hash 0x714C1031).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return
/// slot.
export!(cdecl, rw_00bb8c00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
