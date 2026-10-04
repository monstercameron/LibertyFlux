// original: 0x00b8c830 GET_FILTER_SAVE_SETTING
/// Script native `GET_FILTER_SAVE_SETTING`.
///
/// Forwards one script argument (a filter index) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8c830(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
