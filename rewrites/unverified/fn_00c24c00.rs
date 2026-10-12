// original: 0x00C24C00 pending

/// Read two three-float vectors through tagged parser calls, pass each
/// coordinate and the leading float to the scalar blend worker, then send
/// the three worker results to the final object call. Return that call's
/// EAX value.
lf_checker_rt::export!(stdcall, rw_00c24c00(weight: u32, first_object: u32, second_object: u32, output_object: u32) -> u32 {
    const FIRST_TAG_VA: u32 = 0x00EC_5790;
    const SECOND_TAG_VA: u32 = 0x00EC_5794;
    const READ_VECTOR: u32 = 1;
    const BLEND_COORDINATE: u32 = 3;
    const APPLY_RESULT: u32 = 4;

    let mut first_vector = [0u32; 3];
    let mut second_vector = [0u32; 3];

    let _ = lf_checker_rt::callee_thiscall!(
        READ_VECTOR,
        u32,
        first_object,
        first_vector.as_mut_ptr() as u32,
        lf_checker_rt::relocated(FIRST_TAG_VA),
    );
    let _ = lf_checker_rt::callee_thiscall!(
        2,
        u32,
        second_object,
        second_vector.as_mut_ptr() as u32,
        lf_checker_rt::relocated(SECOND_TAG_VA),
    );

    let x = lf_checker_rt::callee_cdecl!(
        BLEND_COORDINATE,
        f32,
        weight,
        first_vector[0],
        second_vector[0],
    );
    let y = lf_checker_rt::callee_cdecl!(
        BLEND_COORDINATE,
        f32,
        weight,
        first_vector[1],
        second_vector[1],
    );
    let z = lf_checker_rt::callee_cdecl!(
        BLEND_COORDINATE,
        f32,
        weight,
        first_vector[2],
        second_vector[2],
    );

    let mut result = [x.to_bits(), y.to_bits(), z.to_bits()];
    lf_checker_rt::callee_thiscall!(
        APPLY_RESULT,
        u32,
        output_object,
        result.as_mut_ptr() as u32,
    )
});
