// original: 0x00BD46A0 UPDATE_PTFX_TINT
//
// Forwards the particle id and the RGBA tint (floats, bitwise) to the
// engine routine. No return value.
export!(cdecl, rw_00bd46a0(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}
