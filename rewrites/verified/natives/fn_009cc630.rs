// original: 0x009cc630 SET_SCRIPT_MIC_LOOK_AT
// SET_SCRIPT_MIC_LOOK_AT: call the engine with (arg0..arg2). Floats as bits.
export!(cdecl, rw_009cc630(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2))
    }
});
