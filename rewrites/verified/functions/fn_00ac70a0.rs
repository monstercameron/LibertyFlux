// original: 0x00AC70A0 streaming_set_notification_flags

/// OR the low byte of the argument into the notification flags, then forward
/// the stored notification mode to the helper callback.
///
/// The flags are at `0x154e048`, and the callback's single cdecl argument is
/// read from `0x154e044`. The callback's EAX result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_00ac70a0(flags: u32) -> u32 {
    const NOTIFICATION_MODE: u32 = 0x0015_4e044;
    const NOTIFICATION_FLAGS: u32 = 0x0015_4e048;
    const CALLBACK: u32 = 1;

    unsafe {
        let mode = lf_checker_rt::global::<u32>(NOTIFICATION_MODE).read();
        let flags_slot = lf_checker_rt::global::<u32>(NOTIFICATION_FLAGS);
        flags_slot.write(flags_slot.read() | (flags & 0xff));
        lf_checker_rt::callee_cdecl!(CALLBACK, u32, mode)
    }
});
