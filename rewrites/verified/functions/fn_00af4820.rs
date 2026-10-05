// original: 0x00AF4820 drawable_sync_guarded (proposed)

/// Lock, refresh a slot object twice, use it, unlock.
///
/// Locks the stream mutex global, runs two refresh callees over the slot
/// object `s`, runs the use callee with (this, `s`), then clears the mutex
/// global. Returns the use callee's result. (`s` is reloaded from its
/// argument slot after the lock call, before that call's stack cleanup.)
///
/// Original: 0x00AF4820 (thiscall, one stack word, four direct callees).
lf_checker_rt::export!(thiscall, rw_00af4820(this: u32, s: u32) -> u32 {
    unsafe {
        const LOCK_CALLEE: u32 = 1;
        const REFRESH1_CALLEE: u32 = 2;
        const REFRESH2_CALLEE: u32 = 3;
        const USE_CALLEE: u32 = 4;
        const MUTEX: u32 = 0x015F8B40;
        lf_checker_rt::callee_cdecl!(LOCK_CALLEE, u32, lf_checker_rt::relocated(MUTEX));
        lf_checker_rt::callee_thiscall!(REFRESH1_CALLEE, u32, s);
        lf_checker_rt::callee_thiscall!(REFRESH2_CALLEE, u32, s);
        let r = lf_checker_rt::callee_thiscall!(USE_CALLEE, u32, this, s);
        (lf_checker_rt::global::<u32>(MUTEX)).write_unaligned(0);
        r
    }
});
