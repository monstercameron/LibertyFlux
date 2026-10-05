// original: 0x008AA440 audio_voice_teardown (proposed)

/// Tear down a voice: drop the shared counter, free its bitset, clear flags.
///
/// If `this+0x3230` is zero there is nothing to do. Otherwise, when
/// `this+0x3231` is nonzero the voice is deactivated first (voice reset
/// `0x8a9580`, then the lock/unlock pair around a decrement of the shared
/// counter `0x115f854`). Then `this+0x3231` is cleared, the bitset at
/// `this+0x28a0` is passed to the free routine (`0x401250`, cdecl) and the
/// slot nulled, and `this+0x3230` is cleared. No return value. Original is
/// thiscall with no stack words (plain `ret`).
lf_checker_rt::export!(thiscall, rw_008AA440(this: u32) -> u32 {
    const RESET: u32 = 1;
    const LOCK: u32 = 2;
    const UNLOCK: u32 = 3;
    const FREE: u32 = 4;
    const LIVE: u32 = 0x3230;
    const ACTIVE: u32 = 0x3231;
    const BITSET: u32 = 0x28a0;
    const HANDLE: u32 = 0x0115_f858;
    const COUNTER: u32 = 0x0115_f854;
    unsafe {
        if ((this + LIVE) as *const u8).read() == 0 {
            return 0;
        }
        if ((this + ACTIVE) as *const u8).read() != 0 {
            lf_checker_rt::callee_thiscall!(RESET, u32, this);
            let h = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(LOCK, u32, h);
            let h2 = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
            let c = lf_checker_rt::global::<u32>(COUNTER);
            (c as *mut u32).write_unaligned((c as *const u32).read_unaligned().wrapping_sub(1));
            lf_checker_rt::callee_cdecl!(UNLOCK, u32, h2);
        }
        ((this + ACTIVE) as *mut u8).write(0);
        let bits = ((this + BITSET) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(FREE, u32, bits);
        ((this + BITSET) as *mut u32).write_unaligned(0);
        ((this + LIVE) as *mut u8).write(0);
    }
    0
});
