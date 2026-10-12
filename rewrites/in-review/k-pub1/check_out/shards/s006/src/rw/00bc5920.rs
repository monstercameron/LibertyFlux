// original: 0x00BC5920 GET_CAR_ANIM_CURRENT_TIME
//
// Forwards the vehicle handle, the two animation names and the output slot
// to the engine query. The handler itself never touches the return slot:
// the engine writes the time through the forwarded output pointer.
export!(cdecl, rw_00bc5920(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}
