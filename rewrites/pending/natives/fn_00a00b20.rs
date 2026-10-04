// original: 0x00a00b20 FREEZE_OBJECT_POSITION
// rw_freeze_object_position: native FREEZE_OBJECT_POSITION (handler 0x00A00B20).
//
// Forwards an object handle plus a bool coerced from nonzero to 1 (low byte of the dead incoming-arg slot reused as the temp).
// NOTE: the original coerces the bool with `setne` into the low byte
// of its own dead incoming-argument stack slot and pushes that whole
// dword to the engine; the pushed value (high bytes = context address)
// is masked in the contract (call_skip); this contract checks `calls`
// but not `stack` (see report).
export!(cdecl, rw_freeze_object_position(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let flag1 = ((*args.add(1) != 0) as u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), flag1);
        ans
    }
});
