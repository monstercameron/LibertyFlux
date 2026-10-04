// original: 0x00d6f510 replay_bar_maybe_notify
/// Refresh the shared notifier unless the game state suppresses it.
///
/// Returns early with the callee-1 sample when it is below `bound`, or with
/// the state word at 0x1037720 when that state is in the suppressed set
/// {2, 7, 8, 11..17}. Otherwise notifies callee 2, then clears bit 0 of the
/// flag byte at +0x398 of the notifier from callee 3 (skipped when callee 3
/// answers null on its first sample). Returns the last callee answer.
lf_checker_rt::export!(thiscall, rw_00d6f510(this_ptr: u32, bound: u32) -> u32 {
    unsafe {
        let t: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if t < bound {
            return t;
        }
        let st = *lf_checker_rt::global::<u32>(0x1037720);
        match st {
            2 | 7 | 8 | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 => return st,
            _ => {}
        }
        let inner = *(((this_ptr as *const u8).add(4)) as *const u32);
        lf_checker_rt::callee_thiscall!(2, u32, inner);
        let hub = lf_checker_rt::relocated(0x103e498);
        let p: u32 = lf_checker_rt::callee_thiscall!(3, u32, hub);
        if p == 0 {
            return 0;
        }
        let q: u32 = lf_checker_rt::callee_thiscall!(3, u32, hub);
        *(((q as *mut u8).add(0x398)) as *mut u8) &= 0xfe;
        q
    }
});
