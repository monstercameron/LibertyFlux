// original: 0x00b8c9a0 GET_MENU_ITEM_ACCEPTED
/// Script native `GET_MENU_ITEM_ACCEPTED` (hash 0x0F322A6C).
///
/// Forwards one script argument to the engine and stores the full 32-bit
/// answer into the return slot.
export!(cdecl, rw_00b8c9a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
