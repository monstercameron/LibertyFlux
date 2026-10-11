// original: 0x009C9720 stream_clear_request_status

/// Forward value to the streaming request manager using its thiscall
/// convention, store zero in the request-status global, and return the
/// manager's EAX result unchanged.
lf_checker_rt::export!(cdecl, rw_009c9720(value: u32) -> u32 {
    unsafe {
        const REQUEST_MANAGER: u32 = 0x0128_2fa8;
        const REQUEST_STATUS: u32 = 0x0129_574c;
        let result = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(REQUEST_MANAGER),
            value,
        );
        lf_checker_rt::global::<u32>(REQUEST_STATUS).write_unaligned(0);
        result
    }
});
