// original: 0x00bb75d0 REQUEST_MODEL
// REQUEST_MODEL: call the engine with arg0. No return value.
export!(cdecl, rw_00bb75d0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a)
    }
});
