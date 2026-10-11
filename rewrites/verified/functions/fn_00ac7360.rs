// original: 0x00AC7360 update_stream_scalar_103f274

/// Store a new scalar when it differs from the current value, then notify the
/// streaming callback with the event id and address of the shared record.
///
/// The compare treats signed zeros as equal and NaNs as changed, matching the
/// original unordered comparison. The outgoing callback receives four cdecl
/// words; the input float is the sole cdecl stack argument.
lf_checker_rt::export!(cdecl, rw_00ac7360(value: f32) -> () {
    const VALUE: u32 = 0x0010_3f274;
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
