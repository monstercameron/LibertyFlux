// original: 0x00AC7460 update_stream_vector_103f2d0

/// Compare four input floats with the current vector, store all four when any
/// component differs or is unordered, and notify the streaming callback.
///
/// This function has no enable gate. Its pointer argument refers to four
/// contiguous floats; signed zeros compare equal and NaNs compare unequal. The
/// callback receives the event id, vector address, selector and priority.
lf_checker_rt::export!(cdecl, rw_00ac7460(values: u32) -> () {
    const VECTOR: u32 = 0x0010_3f2d0;
    const EVENT: u32 = 0x0015_4e038;
    const CALLBACK: u32 = 1;

    unsafe {
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
