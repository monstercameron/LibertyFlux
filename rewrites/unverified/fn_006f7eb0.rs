// original: 0x006F7EB0 mainloop_timer_format_buffer_returning_self

/// Clear the first byte of the destination and, when either source argument is
/// nonzero, call the shared formatter with the destination, relocated format
/// string, and both source words. Return the destination pointer after the
/// call; the method uses thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_006f7eb0(destination: u32, first_value: u32, second_value: u32) -> u32 {
    const FORMAT_STRING_FILE_VA: u32 = 0x00fae0f0;
    unsafe { (destination as *mut u8).write(0); }
    if first_value != 0 || second_value != 0 {
        let format_string = lf_checker_rt::relocated(FORMAT_STRING_FILE_VA);
        let _format_result: u32 = lf_checker_rt::callee_cdecl!(1, u32, destination, format_string, first_value, second_value);
    }
    destination
});
