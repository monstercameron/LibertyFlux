// original: 0x00AF5EC0 read_three_float_properties

/// Read three named float properties from a context and write their values
/// consecutively to the caller's output array. The intercepted helper is a
/// thiscall routine returning f32 on the x87 stack; each result is stored as
/// its original 32-bit representation.
lf_checker_rt::export!(cdecl, rw_00AF5EC0(context: u32, output: u32) -> u32 {
    const PROPERTY_KEYS: [u32; 3] = [0x00EA_8B80, 0x00EA_8B84, 0x00EA_8B88];
    const FLOAT_STRIDE: u32 = 4;

    let first = lf_checker_rt::callee_thiscall!(
        1,
        f32,
        context,
        lf_checker_rt::relocated(PROPERTY_KEYS[0]),
        0,
    );
    let second = lf_checker_rt::callee_thiscall!(
        1,
        f32,
        context,
        lf_checker_rt::relocated(PROPERTY_KEYS[1]),
        0,
    );
    let third = lf_checker_rt::callee_thiscall!(
        1,
        f32,
        context,
        lf_checker_rt::relocated(PROPERTY_KEYS[2]),
        0,
    );

    unsafe {
        (output as *mut u32).write_volatile(first.to_bits());
        (output.wrapping_add(FLOAT_STRIDE) as *mut u32).write_volatile(second.to_bits());
        (output.wrapping_add(FLOAT_STRIDE * 2) as *mut u32).write_volatile(third.to_bits());
    }
    0
});
