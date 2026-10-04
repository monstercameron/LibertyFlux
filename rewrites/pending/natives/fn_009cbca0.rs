// original: 0x009cbca0 GET_STATIC_EMITTER_PLAYTIME
// rw_get_static_emitter_playtime: native GET_STATIC_EMITTER_PLAYTIME (handler 0x009CBCA0).
//
// Forwards an emitter index to the playtime query, stores the result in the return slot.
export!(cdecl, rw_get_static_emitter_playtime(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0));
        *(*ctx as *mut u32) = ans;
        ans
    }
});
