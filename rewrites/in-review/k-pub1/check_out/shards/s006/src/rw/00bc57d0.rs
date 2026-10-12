// original: 0x00BC57D0 FIND_TIME_POSITION_IN_RECORDING
//
// Forwards the vehicle handle; the engine returns the recording time as an
// x87 float. The original spills it through its own incoming context slot
// before copying it to the return slot; both writes are reproduced (the
// slot offset is measured per build, see rw_00b9e340).
export!(cdecl, rw_00bc57d0(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let result = __lf_callee_cdecl!(1, u32, *args);
        *retslot_of(ctx) = result;
        result
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}

/// Read the return-slot pointer out of a native call context (r-n15).
#[allow(dead_code)]
#[inline(always)]
unsafe fn retslot_of(ctx: *mut u8) -> *mut u32 {
    *(ctx as *const *mut u32)
}
