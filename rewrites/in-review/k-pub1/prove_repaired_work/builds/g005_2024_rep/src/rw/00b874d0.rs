// original: 0x00B874D0 SET_CAM_NEAR_CLIP
//
// Forwards the camera handle and the new near-clip distance (a float, moved
// bitwise) to the engine setter. No return value.
export!(cdecl, rw_00b874d0(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}
