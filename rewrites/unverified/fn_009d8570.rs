// original: 0x009D8570 streaming_parse_integer_and_three_floats

/// Call the cdecl parser with the input text and its four-output format. The
/// parser receives one integer slot followed by three float slots; those local
/// results are not read by this wrapper. Return the parser's EAX result.
lf_checker_rt::export!(cdecl, rw_009d8570(input_text: u32) -> u32 {
    const FORMAT_STRING_VA: u32 = 0x00e9_6f18;
    let mut parsed_fields = [0u32; 4];
    let output_ptr = parsed_fields.as_mut_ptr() as u32;
    let output_0 = output_ptr;
    let output_1 = output_ptr.wrapping_add(4);
    let output_2 = output_ptr.wrapping_add(8);
    let output_3 = output_ptr.wrapping_add(12);
    let format = lf_checker_rt::relocated(FORMAT_STRING_VA);

    lf_checker_rt::callee_cdecl!(1, u32, input_text, format, output_0, output_1, output_2, output_3)
});
