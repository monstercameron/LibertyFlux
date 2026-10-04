// original: 0x00cf7f30 climb_task_ready_check (proposed)

/// Reports whether a climb task is ready: zero when the flag byte at `+0xcb`
/// is clear, otherwise the readiness probe's return value with only its low
/// byte replaced (1 when the probe's low byte is nonzero, 0 when it is zero).
/// The upper three bytes are leftovers the rewrite cannot observe (the
/// caller's entry eax on the early-out path, the probe stub's on the call
/// path), so the contract compares only the low byte.
///
/// Original: 0x00cf7f30 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf7f30(this: u32) -> u32 {
    unsafe {
        const FLAG_OFFSET: u32 = 0xcb;
        const PROBE_CALLEE: u32 = 1;
        let flag = ((this + FLAG_OFFSET) as *const u8).read();
        if flag == 0 {
            return 0;
        }
        let probe = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, this);
        if (probe & 0xff) == 0 {
            probe & 0xffff_ff00
        } else {
            (probe & 0xffff_ff00) | 1
        }
    }
});
