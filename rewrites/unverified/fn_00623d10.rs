// original: 0x00623D10 network_saved_identity_matches_request

/// Match a request while the receiver is in signed status range 2 through 3.
/// The first request guard compares the receiver's saved key words at 0xbf0
/// and 0xbf4 with its active key words at 0xc30 and 0xc34. On a match, the
/// validation helper receives the request's words at 0x1c and 0x20 plus the
/// receiver. If its low byte is nonzero, a route helper receives the request
/// pointer through the receiver's embedded route state at 0xd0. The result
/// is the route helper's low byte. The first stack argument is the payload;
/// the second is the route request.
lf_checker_rt::export!(thiscall, rw_00623d10(this: u32, payload: u32, route_request: u32) -> u32 {
    unsafe {
        const STATUS: u32 = 0x50;
        const SAVED_KEY_LO: u32 = 0xbf0;
        const SAVED_KEY_HI: u32 = 0xbf4;
        const ACTIVE_KEY_LO: u32 = 0xc30;
        const ACTIVE_KEY_HI: u32 = 0xc34;
        const ROUTE_STATE: u32 = 0xd0;
        const REQUEST_KEY_LO: u32 = 0x1c;
        const REQUEST_KEY_HI: u32 = 0x20;
        let status = (this.wrapping_add(STATUS) as *const u32).read_unaligned() as i32;
        if !(2..=3).contains(&status)
            || (this.wrapping_add(SAVED_KEY_LO) as *const u32).read_unaligned()
                != (this.wrapping_add(ACTIVE_KEY_LO) as *const u32).read_unaligned()
            || (this.wrapping_add(SAVED_KEY_HI) as *const u32).read_unaligned()
                != (this.wrapping_add(ACTIVE_KEY_HI) as *const u32).read_unaligned()
        {
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
