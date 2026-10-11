// original: 0x00AC7320 update_stream_scalar_103f264

/// Store a new scalar when it differs from the current value, then notify the
/// streaming callback with the event id and address of the backing record.
///
/// Equality follows the original unordered compare: positive and negative
/// zero compare equal, while a NaN always takes the update path. The call is
/// cdecl with four words, and the function itself is cdecl with one float word.
lf_checker_rt::export!(cdecl, rw_00ac7320(value: f32) -> () {
    const VALUE: u32 = 0x0010_3f264;
    const RECORD: u32 = 0x0010_3f260;
    const EVENT: u32 = 0x0015_4e014;
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
