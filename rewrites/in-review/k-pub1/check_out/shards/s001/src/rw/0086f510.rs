// original: 0x0086F510 TO_FLOAT
/// F25 TO_FLOAT: converts args[0] from int32 to float32 (round to nearest,
/// like cvtdq2ps) and stores it in the return slot. Leaf, no calls.
export!(cdecl, rn10_to_float(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        *ret = ((*args as i32) as f32).to_bits();
        ret as u32
    }
});

/// Read the return slot and argument pointers from a call context. (r-n10.)
#[inline(always)]
pub unsafe fn ctx_parts(ctx: *const u32) -> (*mut u32, *const u32) {
    ((*ctx.add(RET_OFF_WORDS)) as *mut u32, (*ctx.add(ARGS_OFF_WORDS)) as *const u32)
}

/// Word offset of the argument-array pointer in the call context. (r-n10.)
pub const ARGS_OFF_WORDS: usize = 2;

/// Word offset of the return-slot pointer in the call context. (r-n10.)
pub const RET_OFF_WORDS: usize = 0;
