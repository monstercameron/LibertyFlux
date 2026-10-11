// original: 0x00DB10A0 format_search_criteria_buffer
/// Formats a cdecl string and argument-list pair into the shared 128-byte
/// output buffer. The formatting helper is called with the buffer, capacity,
/// format value and a pointer to a one-word argument-list value. That pointer
/// is stack-local, so only its pointed-to word is compared by the contract.
/// A negative signed result terminates byte 127; a nonnegative result below
/// 128 terminates at that index and returns the buffer. Results at least 128
/// tail-call the overflow handler. The global buffer address is relocated.
lf_checker_rt::export!(cdecl, rw_00DB10A0(format_value: u32, argument_list: u32) -> u32 {
    const OUTPUT_BUFFER_VA: u32 = 0x017A65D8;
    const OUTPUT_CAPACITY: u32 = 0x80;
    const LAST_OUTPUT_INDEX: usize = 0x7F;
    const FORMATTER_ID: u32 = 1;
    const OVERFLOW_HANDLER_ID: u32 = 2;

    let output_address = lf_checker_rt::relocated(OUTPUT_BUFFER_VA);
    let mut argument_list_copy = argument_list;
    let argument_list_pointer = (&mut argument_list_copy as *mut u32) as usize as u32;
    let formatted_length = lf_checker_rt::callee_cdecl!(
        FORMATTER_ID,
        u32,
        output_address,
        OUTPUT_CAPACITY - 1,
        format_value,
        argument_list_pointer,
    );

    let output_buffer = lf_checker_rt::global::<u8>(OUTPUT_BUFFER_VA);
    if (formatted_length as i32) < 0 {
        // SAFETY: the declared global buffer has 128 writable bytes.
        unsafe { output_buffer.add(LAST_OUTPUT_INDEX).write(0); }
        return output_address;
    }
    if formatted_length >= OUTPUT_CAPACITY {
        return lf_checker_rt::callee_cdecl!(OVERFLOW_HANDLER_ID, u32, );
    }

    // SAFETY: the preceding bound check keeps this index inside the buffer.
    unsafe { output_buffer.add(formatted_length as usize).write(0); }
    output_address
});
