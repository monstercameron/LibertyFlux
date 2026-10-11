// original: 0x00922210 submit_adjusted_float_when_target_present

/// Forward a transformed float to the final helper when the target is non-null.
///
/// The five cdecl stack arguments are two float bit patterns, a target pointer,
/// and two further float bit patterns. The first helper returns an unsigned
/// word that the original converts as a signed 32-bit integer to `f32`; the
/// second helper returns an `f32` on the x87 stack. The function multiplies
/// that second result by the signed count, subtracts the product from the
/// second input float, then passes the first input and the result to the last
/// helper. A null target skips all three calls. The helper return values are
/// used only as intermediates; this routine has no specified return value.
///
/// Calling convention: cdecl, five stack words. The original uses the target
/// and following incoming argument slots as temporary storage; the values are
/// observed in the final helper call and in the computed float.
lf_checker_rt::export!(cdecl, rw_00922210(
    first_float_bits: u32,
    second_float_bits: u32,
    target: u32,
    helper_float_bits: u32,
    helper_mode: u32,
) -> u32 {
    if target == 0 {
        return 0;
    }

    const COUNT_CALLEE: u32 = 1;
    const FLOAT_CALLEE: u32 = 2;
    const SUBMIT_CALLEE: u32 = 3;

    let signed_count = lf_checker_rt::callee_cdecl!(
        COUNT_CALLEE,
        u32,
        first_float_bits,
        second_float_bits,
        target,
        0u32,
    ) as i32;

    let helper_float = lf_checker_rt::callee_cdecl!(
        FLOAT_CALLEE,
        f32,
        target,
        helper_float_bits,
        helper_mode,
    );

    let count_float = signed_count as f32;
    let product = {
        let left = core::hint::black_box(helper_float);
        let right = core::hint::black_box(count_float);
        left * right
    };
    let adjusted_float = {
        let left = core::hint::black_box(f32::from_bits(second_float_bits));
        let right = core::hint::black_box(product);
        left - right
    };

    let _ignored = lf_checker_rt::callee_cdecl!(
        SUBMIT_CALLEE,
        u32,
        first_float_bits,
        adjusted_float.to_bits(),
    );

    0
});
