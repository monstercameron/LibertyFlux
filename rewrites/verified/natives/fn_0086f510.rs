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
