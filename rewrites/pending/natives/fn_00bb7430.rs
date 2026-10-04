// original: 0x00bb7430 IS_THIS_MODEL_A_PED
// IS_THIS_MODEL_A_PED: engine(arg0), zero-extend low byte into return slot.
export!(cdecl, rw_00bb7430(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a);
        *ret_of(ctx) = r & 0xFF;
        r
    }
});
