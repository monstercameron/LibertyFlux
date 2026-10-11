// original: 0x00C10C00 streaming_request_submit_payload_pair

/// Submit an inactive stream record to the shared queue with pointers to its
/// two adjacent payload regions and a sentinel final argument. The receiver's
/// byte at +1 must be zero. The helper owns three stack arguments and uses the
/// relocated queue object as ECX.
lf_checker_rt::export!(thiscall, rw_00c10c00(this: u32) -> u32 {
    unsafe {
        const SUBMIT_ID: u32 = 1;
        const QUEUE_VA: u32 = 0x0128_31E4;
        if ((this + 1) as *const u8).read() == 0 {
            let first_payload = this + 0x10;
            let second_payload = this + 0x20;
            let _ = lf_checker_rt::callee_thiscall!(
                SUBMIT_ID, u32, lf_checker_rt::relocated(QUEUE_VA),
                first_payload, second_payload, u32::MAX
            );
        }
    }
    0
});
