// original: 0x00AC7270 update_stream_vector_154e0f0

/// When enabled, compare four input floats against the current vector, store
/// all four on the first differing or unordered component, and notify the
/// streaming callback.
///
/// The input is a pointer to four contiguous floats. Signed zeros compare
/// equal and NaNs compare unequal. The callback receives the event id, vector
/// address, selector and priority as four cdecl words.
lf_checker_rt::export!(cdecl, rw_00ac7270(values: u32) -> () {
    const ENABLED: u32 = 0x0010_3f254;
    const VECTOR: u32 = 0x0015_4e0f0;
    const EVENT: u32 = 0x0015_4e020;
    const CALLBACK: u32 = 1;

    unsafe {
        if lf_checker_rt::global::<u8>(ENABLED).read() == 0 {
            return;
        }
        let source = values as *const f32;
        let target = lf_checker_rt::global::<f32>(VECTOR);
        let mut changed = false;
        for index in 0..4 {
            if source.add(index).read_unaligned() != target.add(index).read() {
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
