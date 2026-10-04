// original: 0x00B986B0 IS_NUMLOCK_ENABLED
// IS_NUMLOCK_ENABLED: takes no arguments; stores the engine answer's low
// byte (zero-extended) into the return slot. The original reloads EAX from
// the context for the store, so it returns the slot pointer, not the answer.
export!(cdecl, rw_fn_b986b0(ctx: *mut u8) -> u32 {
    unsafe {
        let ans: u32 = callee_cdecl!(1, u32,);
        let ret = *(ctx as *mut *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});
