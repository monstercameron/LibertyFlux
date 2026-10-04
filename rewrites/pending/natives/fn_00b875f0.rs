// original: 0x00B875F0 SET_CAM_PROPAGATE
// SET_CAM_PROPAGATE: forward (cam, propagate?) with the bool coerced through
// the incoming stack slot as above.
export!(cdecl, rw_00B875F0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, coerced(ctx, *a.add(1)))
    }
});
