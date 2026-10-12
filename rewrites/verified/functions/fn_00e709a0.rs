// original: 0x00E709A0 initialize_record_set_00e709a0

/// Calls the intercepted thiscall constructor 8 times with no
/// stack arguments. Receiver addresses descend from 0x01B4A358 by
/// 328 bytes per call, and the final callee result is returned.
lf_checker_rt::export!(cdecl, rw_00e709a0() -> u32 {
    const RECEIVER_BASE_VA: u32 = 0x01B4A358;
    const RECEIVER_STRIDE: u32 = 0x148;
    const CALL_COUNT: u32 = 8;

    let mut last_result = 0;
    for index in 0..CALL_COUNT {
        let receiver_va = RECEIVER_BASE_VA - RECEIVER_STRIDE * (index + 1);
        let receiver = lf_checker_rt::relocated(receiver_va);
        last_result = lf_checker_rt::callee_thiscall!(1, u32, receiver);
    }
    last_result
});
