// original: 0x00ba0d40 REMOVE_GROUP
// rw_remove_group: native REMOVE_GROUP (handler 0x00BA0D40).
//
// Forwards one group handle to the group remover. No return slot.
export!(cdecl, rw_remove_group(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
