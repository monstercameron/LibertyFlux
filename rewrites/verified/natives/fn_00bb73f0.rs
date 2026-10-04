// original: 0x00bb73f0 IS_MODEL_IN_CDIMAGE
// IS_MODEL_IN_CDIMAGE: engine(arg0), zero-extend low byte into return slot.
export!(cdecl, rw_00bb73f0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a);
        *ret_of(ctx) = r & 0xFF;
        r
    }
});
