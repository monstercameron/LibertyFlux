// original: 0x00ba0ce0 REMOVE_CHAR_ELEGANTLY
// rw_remove_char_elegantly: native REMOVE_CHAR_ELEGANTLY (handler 0x00BA0CE0).
//
// Forwards one char handle to the elegant remover. No return slot.
export!(cdecl, rw_remove_char_elegantly(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
