// original: 0x00bd9430 SET_OBJECT_STOP_CLONING
// rw_set_object_stop_cloning: native SET_OBJECT_STOP_CLONING (handler 0x00BD9430).
//
// Forwards an object handle plus a bool coerced from nonzero to 1 (low byte of the dead incoming-arg slot reused as the temp).
// NOTE: the original coerces the bool with `setne` into the low byte
// of its own dead incoming-argument stack slot and pushes that whole
// dword to the engine; the pushed value (high bytes = context address)
// is masked in the contract (call_skip); this contract checks `calls`
// but not `stack` (see report).
export!(cdecl, rw_set_object_stop_cloning(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let flag1 = ((*args.add(1) != 0) as u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), flag1);
        ans
    }
});
