// original: 0x00AC7090 stream_notification_nibble

/// OR the low four bits of a byte argument into the notification flag byte.
///
/// The function is cdecl with one stack word. It masks the argument to a
/// nibble, then updates byte `0x154e042`; all other bits already stored there
/// are preserved. The return register is not part of the function's result.
lf_checker_rt::export!(cdecl, rw_00ac7090(_flags: u32) -> () {
    const NOTIFICATION_FLAGS: u32 = 0x0015_4e042;
    const LOW_NIBBLE: u8 = 0x0f;

    unsafe {
        let flags = lf_checker_rt::global::<u8>(NOTIFICATION_FLAGS);
        let incoming = (_flags as u8) & LOW_NIBBLE;
        flags.write(flags.read() | incoming);
    }
});
