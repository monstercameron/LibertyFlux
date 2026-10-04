// original: 0x0086f280 EXP
//
// Loads the float argument and calls the CRT exponential; the callee takes
// its input in a vector register and returns the result the same way (both
// unobservable under interception), and the handler stores the result word
// into the return slot.
export!(cdecl, rw_0086f280(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let _input = *args;
        let result = callee_cdecl!(1, u32,);
        *retslot_of(ctx) = result;
        result
    }
});
