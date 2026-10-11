// original: 0x009D9650 streaming_parse_six_float_values

/// Parse six float fields into temporary words, then pass those words to a
/// second cdecl helper in reverse field order. The parser's count is ignored;
/// the second helper's EAX is returned. Float values are forwarded by their
/// exact 32-bit representations without arithmetic.
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
        output_ptr,
        output_ptr.wrapping_add(4),
        output_ptr.wrapping_add(8),
        output_ptr.wrapping_add(12),
        output_ptr.wrapping_add(16),
        output_ptr.wrapping_add(20)
    );
    lf_checker_rt::callee_cdecl!(2, u32, parsed_values[5], parsed_values[4], parsed_values[3], parsed_values[2], parsed_values[1], parsed_values[0])
});
