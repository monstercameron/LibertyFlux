// original: 0x00623CC0 network_status_request_acceptance

/// Accept a network request only while the receiver is in the signed status
/// range 2 through 3. The first stack argument points to a payload record and
/// the second points to a route request. A validation helper is called with
/// the route request as `this`, the payload's words at 0x1c/0x20, and the
/// receiver. On success, a route helper on the receiver's embedded state
/// receives the route request. Both helpers are intercepted external calls;
/// the result is the second helper's low byte as a boolean. The receiver's
/// status is at byte offset 0x50 and its route state is at 0xd0.
lf_checker_rt::export!(thiscall, rw_00623cc0(this: u32, payload: u32, route_request: u32) -> u32 {
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
            route_request,
            (payload.wrapping_add(REQUEST_KEY_LO) as *const u32).read_unaligned(),
            (payload.wrapping_add(REQUEST_KEY_HI) as *const u32).read_unaligned(),
            this,
        );
        if accepted & 0xff == 0 {
            return 0;
        }
        let routed = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(ROUTE_STATE), route_request);
        u32::from(routed & 0xff != 0)
    }
});
