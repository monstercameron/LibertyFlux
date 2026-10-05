// original: 0x00a08bb0 pool_object_set_tracked (proposed)
/// Mark a pool object tracked or untracked and notify its tracker.
///
/// With `track` set: clear bit 3 at object+0x118 and enable the tracker at
/// object+0x80 (flag 1), then attach it. With `track` clear: set bit 3,
/// disable the tracker (flag 0), detach it, set bit 3 at object+0x24, and
/// tail-call the untrack notifier. Returns the last callee's answer. Cdecl.
lf_checker_rt::export!(cdecl, rw_00a08bb0(obj: u32, track: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x118;
        const TRACKER_OFF: u32 = 0x80;
        const MODE_OFF: u32 = 0x24;
        const TRACK_BIT: u32 = 8;
        const SET_FLAG: u32 = 0;
        const ATTACH: u32 = 1;
        const DETACH: u32 = 2;
        const NOTIFY: u32 = 3;
        if (track & 0xff) != 0 {
            let s = (obj + STATE_OFF) as *mut u32;
            s.write_unaligned(s.read_unaligned() & !TRACK_BIT);
            let tracker = obj + TRACKER_OFF;
            let _: u32 = lf_checker_rt::callee_thiscall!(SET_FLAG, u32, tracker, 1u32);
            lf_checker_rt::callee_cdecl!(ATTACH, u32, tracker)
        } else {
            let s = (obj + STATE_OFF) as *mut u32;
            s.write_unaligned(s.read_unaligned() | TRACK_BIT);
            let tracker = obj + TRACKER_OFF;
            let _: u32 = lf_checker_rt::callee_thiscall!(SET_FLAG, u32, tracker, 0u32);
            let _: u32 = lf_checker_rt::callee_cdecl!(DETACH, u32, tracker);
            let m = (obj + MODE_OFF) as *mut u32;
            m.write_unaligned(m.read_unaligned() | TRACK_BIT);
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, obj)
        }
    }
});
