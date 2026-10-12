// original: 0x00BD3C20 DRAW_SPHERE
//
// Forwards position (x, y, z) and radius to the debug-sphere renderer.
// The floats travel as themselves through stack slots; the moves are bitwise.
export!(cdecl, rw_00bd3c20(ctx: *mut u8) -> u32 {
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
