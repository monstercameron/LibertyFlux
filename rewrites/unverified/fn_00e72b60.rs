// original: 0x00E72B60 initialize_animation_vector_pair_array
/// For a nonzero 16-bit count, initialize each array item at a 0x30-byte stride and then send one notification for the array base.
lf_checker_rt::export!(cdecl, rw_fn_00E72B60() -> () {
    unsafe {
        const VECTOR_ARRAY: u32 = 0x016DD65C;
        const VECTOR_COUNT: u32 = 0x016DD662;
        const VECTOR_STRIDE: u32 = 0x30;

        let vector_count = lf_checker_rt::global::<u16>(VECTOR_COUNT).read_unaligned();
        if vector_count == 0 {
            return;
        }
        let array = lf_checker_rt::global::<u32>(VECTOR_ARRAY).read_unaligned();
        let mut vector = array;
        let helper_count = vector_count;
        for _ in 0..helper_count {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, vector);
            vector = vector.wrapping_add(VECTOR_STRIDE);
        }
        let _ = lf_checker_rt::callee_cdecl!(2, u32, array);
    }
});
