// original: 0x00bb6560 NativeImpl_REGISTER_BEST_POSITION
// rw_register_best_position: native REGISTER_BEST_POSITION (handler 0x00BB6560).
//
// Forwards two ints to the race-best registrar. No return slot.
export!(cdecl, rw_register_best_position(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
