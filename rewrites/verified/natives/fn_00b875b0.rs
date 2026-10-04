// original: 0x00b875b0 SET_CAM_POS
// SET_CAM_POS: call the engine with (arg0..arg3). Floats pass through as bits.
export!(cdecl, rw_00b875b0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), *a.add(3))
    }
});
