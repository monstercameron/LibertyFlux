// original: 0x00B949B0 IS_MEMORY_CARD_IN_USE
//
// Calls the save-system query with no arguments and stores its zero-extended
// low byte into the return slot.
export!(cdecl, rw_00b949b0(ctx: *mut u8) -> u32 {
    unsafe {
        let result = __lf_callee_cdecl!(1, u32,);
        *retslot_of(ctx) = result & 0xFF;
        result
    }
});

/// Read the return-slot pointer out of a native call context (r-n15).
#[allow(dead_code)]
#[inline(always)]
unsafe fn retslot_of(ctx: *mut u8) -> *mut u32 {
    *(ctx as *const *mut u32)
}
