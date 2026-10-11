// original: 0x00C119A0 streaming_request_forward_when_flagged

/// Forward a flagged stream request to the next handler. When bit zero of
/// byte +3 in `request` is set, the handler receives the request pointer and
/// the second stack argument and its u32 result is returned. Otherwise the
/// request pointer itself is returned. The original callee cleans both stack
/// arguments.
lf_checker_rt::export!(stdcall, rw_00c119a0(request: u32, context: u32) -> u32 {
    unsafe {
        const FORWARD_ID: u32 = 1;
        let flags = ((request + 3) as *const u8).read();
        if flags & 1 != 0 {
            lf_checker_rt::callee_stdcall!(FORWARD_ID, u32, request, context)
        } else {
            request
        }
    }
});
