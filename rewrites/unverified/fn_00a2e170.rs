// original: 0x00a2e170 save_vector_stage_globals

/// Copy four raw vector words and one scalar word to their staging globals,
/// then set the byte flag. No floating-point arithmetic is performed. The
/// cdecl function leaves EAX unspecified, so only its memory effects are checked.
lf_checker_rt::export!(cdecl, rw_a2e170(vector: u32, scalar: f32) -> () {
    const VECTOR_GLOBAL: u32 = 0x12dde60;
    const SCALAR_GLOBAL: u32 = 0x12dd8c4;
    const READY_BYTE: u32 = 0x12dd8c0;
    unsafe {
        let input = vector as *const u32;
        for index in 0..4 {
            let value = input.add(index).read_unaligned();
            lf_checker_rt::global::<u32>(VECTOR_GLOBAL).add(index).write_unaligned(value);
        }
        lf_checker_rt::global::<u32>(SCALAR_GLOBAL).write_unaligned(scalar.to_bits());
        (lf_checker_rt::global::<u8>(READY_BYTE)).write_unaligned(1);
    }
});
