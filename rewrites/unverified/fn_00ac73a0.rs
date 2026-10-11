// original: 0x00AC73A0 update_stream_scalar_103f27c

/// Store a new scalar at `0x103f27c` when it differs from the current value,
/// then notify the streaming callback using the shared record at
/// `0x103f270`.
///
/// Signed zeros compare equal and NaNs compare unequal, as with the original
/// SSE comparison. The notification is a four-word cdecl call.
lf_checker_rt::export!(cdecl, rw_00ac73a0(value: f32) -> () {
    const VALUE: u32 = 0x0010_3f27c;
    const RECORD: u32 = 0x0010_3f270;
    const EVENT: u32 = 0x0015_4e018;
    const CALLBACK: u32 = 1;

    unsafe {
        let slot = lf_checker_rt::global::<f32>(VALUE);
        if value != slot.read() {
            slot.write(value);
            let event = lf_checker_rt::global::<u32>(EVENT).read();
            let record = lf_checker_rt::relocated(RECORD);
            let _ = lf_checker_rt::callee_cdecl!(CALLBACK, u32, event, record, 1u32, 5u32);
        }
    }
});
