// original: 0x009d9650 streaming_parse_six_float_values
/// Parses six 32-bit fields from the input text, then forwards the parsed
/// float bit patterns in the order consumed by the downstream helper. The
/// parser writes through six caller-stack pointers; each output is modeled
/// as a local word and passed to the helper after parsing. The wrapper is
/// cdecl and returns the helper's 32-bit result.
lf_checker_rt::export!(cdecl, rw_009d9650(input_text: u32) -> u32 {
    const FORMAT_STRING_VA: u32 = 0x00e9_6ecc;
    let mut parsed_values = [0u32; 6];
    let output_ptr = parsed_values.as_mut_ptr() as u32;
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
    lf_checker_rt::callee_cdecl!(
        2,
        u32,
        parsed_values[5],
        parsed_values[4],
        parsed_values[3],
        parsed_values[2],
        parsed_values[1],
        parsed_values[0]
    )
});
