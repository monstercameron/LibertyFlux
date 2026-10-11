// original: 0x00DA38F0 CPedShelterAttractor::vf4

/// Converts a signed random integer to a scaled angle and writes it through
/// the second stack argument. The first explicit argument is unused. The
/// conversion is signed, followed by multiplication by the two read-only
/// scale constants in that order; the output pointer is returned in EAX.
lf_checker_rt::export!(thiscall, rw_00da38f0(_self_ptr: u32, _unused: u32, output_ptr: u32) -> u32 {
    const RANDOM_UNIT_SCALE_BITS: u32 = 0x3800_0100;
    const FULL_TURN_BITS: u32 = 0x40c9_0fdb;

    #[inline(never)]
    fn multiply_in_original_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    let random_bits = lf_checker_rt::callee_cdecl!(1, u32,);
    let signed_random = random_bits as i32 as f32;
    let unit_fraction = multiply_in_original_order(
        signed_random,
        f32::from_bits(RANDOM_UNIT_SCALE_BITS),
    );
    let angle = multiply_in_original_order(unit_fraction, f32::from_bits(FULL_TURN_BITS));
    unsafe { (output_ptr as *mut f32).write_unaligned(angle) };
    output_ptr
});
