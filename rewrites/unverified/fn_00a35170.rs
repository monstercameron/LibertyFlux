// original: 0x00a35170 vehicle_bank_refresh (proposed)

/// Refresh the bank object through three helper calls, or return its head.
///
/// When the word at `obj + 0x4` is zero, answers the head word `obj[0]`
/// with no calls. Otherwise initialises the helper (id 1, thiscall/0) on
/// `obj + 0xfe0`, feeds it (id 2, thiscall/2) with `(obj + 0x620, obj[0x4])`,
/// resolves the head (id 3, thiscall/1) on `(obj + 0xfe0, x)`, stores the
/// answer back to `obj[0]` and returns it. Cdecl/2, returns EAX.
lf_checker_rt::export!(cdecl, rw_00a35170(obj: u32, x: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x0;
        const COUNT: u32 = 0x4;
        const FEED_OFF: u32 = 0x620;
        const HELPER_OFF: u32 = 0xFE0;
        const INIT: u32 = 1;
        const FEED: u32 = 2;
        const RESOLVE: u32 = 3;
        let count = core::ptr::read_unaligned((obj + COUNT) as *const u32);
        if count == 0 {
            return core::ptr::read_unaligned((obj + HEAD) as *const u32);
        }
        let helper = obj.wrapping_add(HELPER_OFF);
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, helper);
        // Push order is (count, feed-pointer), so the feed pointer is arg 0.
        let _: u32 = lf_checker_rt::callee_thiscall!(FEED, u32, helper, obj.wrapping_add(FEED_OFF), count);
        let head: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, helper, x);
        core::ptr::write_unaligned((obj + HEAD) as *mut u32, head);
        head
    }
});
