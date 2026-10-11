// original: 0x009d8210 streaming_parse_and_initialize_shared_object
/// Parses five integer fields and one float field from the input text. The
/// last integer's low two bits become separate flags; four other integers,
/// the float word, those flags, and the second cdecl argument are forwarded
/// in the original order to the shared object's thiscall method. Parser
/// outputs are modeled in a six-word local array. The wrapper returns the
/// method's 32-bit result.
lf_checker_rt::export!(cdecl, rw_009d8210(input_text: u32, context: u32) -> u32 {
    const FORMAT_STRING_VA: u32 = 0x00e9_6b74;
    const STREAMING_OBJECT_VA: u32 = 0x0117_7a80;
    let mut parsed_fields = [0u32; 6];
    let output_ptr = parsed_fields.as_mut_ptr() as u32;
    let format = lf_checker_rt::relocated(FORMAT_STRING_VA);

    let _parsed = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        input_text,
        format,
        output_ptr.wrapping_add(20),
        output_ptr.wrapping_add(16),
        output_ptr.wrapping_add(12),
        output_ptr.wrapping_add(8),
        output_ptr.wrapping_add(4),
        output_ptr
    );
    let integer_flags = parsed_fields[0];
    let first_float = parsed_fields[3];
    let low_flag = integer_flags & 1;
    let high_flag = (integer_flags >> 1) & 1;
    let object = lf_checker_rt::relocated(STREAMING_OBJECT_VA);

    lf_checker_rt::callee_thiscall!(
        2,
        u32,
        object,
        context,
        parsed_fields[5],
        parsed_fields[4],
        parsed_fields[2],
        parsed_fields[1],
        first_float,
        low_flag,
        high_flag
    )
});
