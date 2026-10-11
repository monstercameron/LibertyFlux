// original: 0x00AC7570 streaming_notify_stream_event

/// Forward the event's stored mode and one caller argument to the streaming
/// notification callback, returning the callback's 32-bit result.
///
/// The mode is read from the global word at `0x154e010`. The callback is a
/// cdecl function with arguments in that order; the wrapper preserves its
/// result in EAX.
lf_checker_rt::export!(cdecl, rw_00ac7570(argument: u32) -> u32 {
    const NOTIFICATION_MODE: u32 = 0x0015_4e010;
    const CALLBACK: u32 = 1;

    unsafe {
        let mode = lf_checker_rt::global::<u32>(NOTIFICATION_MODE).read();
        lf_checker_rt::callee_cdecl!(CALLBACK, u32, mode, argument)
    }
});
