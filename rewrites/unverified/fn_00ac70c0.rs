// original: 0x00AC70C0 update_stream_vector_if_enabled

/// When the enable byte is nonzero, compare four input floats against the
/// current vector, store all four on the first differing or unordered
/// component, and notify the streaming callback.
///
/// The input is a pointer to four contiguous `f32` values. A disabled vector
/// leaves all globals and calls unchanged. Equality treats signed zeros as
/// equal and NaNs as different. The callback receives its event, the vector
/// address, a selector and a priority through a cdecl call.
lf_checker_rt::export!(cdecl, rw_00ac70c0(values: u32) -> () {
    const ENABLED: u32 = 0x0010_3f254;
    const VECTOR: u32 = 0x0010_3f2b0;
    const EVENT: u32 = 0x0015_4e02c;
    const CALLBACK: u32 = 1;

    unsafe {
        if lf_checker_rt::global::<u8>(ENABLED).read() == 0 {
            return;
        }
        let source = values as *const f32;
        let target = lf_checker_rt::global::<f32>(VECTOR);
        let mut changed = false;
        for index in 0..4 {
            let incoming = source.add(index).read_unaligned();
            if incoming != target.add(index).read() {
                changed = true;
                break;
            }
        }
        if changed {
            for index in 0..4 {
                target.add(index).write_unaligned(source.add(index).read_unaligned());
            }
            let event = lf_checker_rt::global::<u32>(EVENT).read();
            let record = lf_checker_rt::relocated(VECTOR);
            let _ = lf_checker_rt::callee_cdecl!(CALLBACK, u32, event, record, 1u32, 5u32);
        }
    }
});
