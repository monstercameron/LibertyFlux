// original: 0x00A8F310 pool_guarded_enable (proposed)

/// Enable the object unless it is already on, then refresh it.
///
/// With the flag byte at `obj+0x76` set there is nothing to do. Otherwise
/// the readiness helper runs on the object; a zero low byte also ends the
/// work. When ready, the refresh helper runs with `n + 1`. Always returns
/// 1, low byte only (upper bits are leftovers).
///
/// Original: cdecl, two stack words (object, count), low byte in AL. Two
/// outgoing calls (readiness: thiscall no args; refresh: thiscall one
/// arg).
lf_checker_rt::export!(cdecl, rw_00A8F310(obj: u32, n: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x76;
        const READY: u32 = 1;
        const REFRESH: u32 = 2;
        if ((obj + FLAG_OFF) as *const u8).read() != 0 {
            return 1;
        }
        let t: u32 = lf_checker_rt::callee_thiscall!(READY, u32, obj);
        if t as u8 == 0 {
            return 1;
        }
        let _: u32 =
            lf_checker_rt::callee_thiscall!(REFRESH, u32, obj, n.wrapping_add(1));
        1
    }
});
