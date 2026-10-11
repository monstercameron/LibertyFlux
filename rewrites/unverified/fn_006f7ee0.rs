// original: 0x006F7EE0 mainloop_timer_format_buffer

/// Clear the first byte of the destination and conditionally call the shared
/// formatter with the destination, relocated format string, and both source
/// words. Preserve the formatter's EAX result when called; otherwise return
/// the first source word already loaded into EAX by the original method.
lf_checker_rt::export!(thiscall, rw_006f7ee0(destination: u32, first_value: u32, second_value: u32) -> u32 {
    const FORMAT_STRING_FILE_VA: u32 = 0x00fae0f0;
    unsafe { (destination as *mut u8).write(0); }
    if first_value != 0 || second_value != 0 {
        lf_checker_rt::callee_cdecl!(1, u32, destination, format_string, first_value, second_value)
    } else {
        first_value
    }
});
