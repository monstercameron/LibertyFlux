// original: 0x00BB7340 GET_NUM_STREAMING_REQUESTS
//
// Calls the engine counter with no arguments and stores the result word
// into the return slot.
export!(cdecl, rw_00bb7340(ctx: *mut u8) -> u32 {
    unsafe {
        let result = callee_cdecl!(1, u32,);
        *retslot_of(ctx) = result;
        result
    }
});

/// Read the return-slot pointer out of a native call context (r-n15).
#[allow(dead_code)]
#[inline(always)]
unsafe fn retslot_of(ctx: *mut u8) -> *mut u32 {
    *(ctx as *const *mut u32)
}
