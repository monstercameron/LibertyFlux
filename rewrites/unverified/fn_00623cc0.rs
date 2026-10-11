// original: 0x00623CC0 network_status_request_acceptance

/// Accept a network request only while the receiver is in the signed status
/// range 2 through 3. The second stack argument points to a request record.
/// The first stack argument is not read. A validation helper receives the
/// request's two key words and the receiver; on success, a route helper on
/// the receiver's embedded route state receives the request pointer. Each
/// helper is an intercepted external call. The result is the route helper's
/// low byte interpreted as a boolean. The receiver layout uses status at
/// byte offset 0x50 and route state at 0xd0; request keys are at 0x1c/0x20.
lf_checker_rt::export!(thiscall, rw_00623cc0(this: u32, _unused: u32, request: u32) -> u32 {
    unsafe {
        const STATUS: u32 = 0x50;
        const ROUTE_STATE: u32 = 0xd0;
        const REQUEST_KEY_LO: u32 = 0x1c;
        const REQUEST_KEY_HI: u32 = 0x20;
        let status = (this.wrapping_add(STATUS) as *const u32).read_unaligned() as i32;
        if !(2..=3).contains(&status) {
            return 0;
        }
        let accepted = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            request,
            (request.wrapping_add(REQUEST_KEY_LO) as *const u32).read_unaligned(),
            (request.wrapping_add(REQUEST_KEY_HI) as *const u32).read_unaligned(),
            this,
        );
        if accepted & 0xff == 0 {
            return 0;
        }
        let routed = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(ROUTE_STATE), request);
        u32::from(routed & 0xff != 0)
    }
});
