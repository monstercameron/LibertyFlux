// original: 0x00B9FB80 IS_CHAR_IN_MELEE_COMBAT
//
// Forwards the character handle; the engine answers in the low byte and the
// handler zero-extends it into the return slot.
export!(cdecl, rw_00b9fb80(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let result = callee_cdecl!(1, u32, *args);
        *retslot_of(ctx) = result & 0xFF;
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
