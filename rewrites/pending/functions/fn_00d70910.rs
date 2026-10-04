// original: 0x00d70910 replay_bar_clamp_bound
/// Clamp a bound against the callee-1 sample per `which`.
///
/// When `which` is 0 stores `min(value, sample)` at +0xB8 and returns 0;
/// when 1 stores `max(value, sample)` at +0xCC and returns it; otherwise
/// stores nothing and returns `which`.
lf_checker_rt::export!(thiscall, rw_00d70910(this_ptr: u32, value: u32, which: u32) -> u32 {
    unsafe {
        let b = this_ptr as *const u8;
        let t: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if which == 0 {
            let d = if value >= t { t } else { value };
            *((b.add(0xb8)) as *mut u32) = d;
            0
        } else if which == 1 {
            let e = if value <= t { t } else { value };
            *((b.add(0xcc)) as *mut u32) = e;
            e
        } else {
            which
        }
    }
});
