// original: 0x00E710C0 initialize_record_set_00e710c0

/// Calls the intercepted thiscall constructor 3 times with no
/// stack arguments. Receiver addresses descend from 0x01161824 by
/// 4 bytes per call, and the final callee result is returned.
lf_checker_rt::export!(cdecl, rw_00e710c0() -> u32 {
    const RECEIVER_BASE_VA: u32 = 0x01161824;
    const RECEIVER_STRIDE: u32 = 0x4;
    const CALL_COUNT: u32 = 3;

    let mut last_result = 0;
    for index in 0..CALL_COUNT {
        let receiver_va = RECEIVER_BASE_VA - RECEIVER_STRIDE * (index + 1);
        let receiver = lf_checker_rt::relocated(receiver_va);
        last_result = lf_checker_rt::callee_thiscall!(1, u32, receiver);
    }
    last_result
});
