// original: 0x009cc6d0 SET_STREAM_PARAMS
//
// Script native handler: forwards a float parameter and an integer to one
// engine function (cdecl/2). The float travels through an SSE register and
// is bit-preserved. No return value stored.
export!(cdecl, rw_009cc6d0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
