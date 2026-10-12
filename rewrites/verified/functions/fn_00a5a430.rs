// original: 0x00A5A430 placement_setup_forward

/// Forwards the object pointer and nine stack words to the full placement
/// setup helper, preserving the original order. The second and third words
/// carry scalar float bits through stack memory; they are moved without
/// arithmetic. The wrapper returns the original object pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00A5A430(
    this: u32,
    first_input: u32,
    axis_x_bits: u32,
    axis_y_bits: u32,
    mode: u32,
    flags: u32,
    extent: u32,
    context: u32,
    item: u32,
    metadata: u32,
) -> u32 {
    let _forwarded_float_bits = (axis_x_bits, axis_y_bits);
    lf_checker_rt::callee_thiscall!(
        1, u32, this,
        first_input, axis_x_bits, axis_y_bits, mode, flags,
        extent, context, item, metadata,
    );
    this
});
