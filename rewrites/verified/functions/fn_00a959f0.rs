// original: 0x00a959f0 stream_reset_all_entries (proposed)

/// Reset a streaming set: two fixed resets, then one pass per entry.
///
/// Callee 1 (thiscall/1 on the object with 0xce) and callee 2 (thiscall/1
/// with 0xc6) run first; then callee 3 (thiscall/1 on the object) runs once
/// per entry index from 0 while the index is below the signed count at
/// `this+0x04` (re-read each pass; nothing runs for a count of zero or less).
///
/// Returns the last per-entry answer, or the second reset's answer when no
/// entry ran. Thiscall: object in ecx, no stack words.
lf_checker_rt::export!(thiscall, rw_00a959f0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x04;
        const RESET1_ARG: u32 = 0xce;
        const RESET2_ARG: u32 = 0xc6;
        const RESET1_CALLEE: u32 = 1;
        const RESET2_CALLEE: u32 = 2;
        const ENTRY_CALLEE: u32 = 3;
        lf_checker_rt::callee_thiscall!(RESET1_CALLEE, u32, this, RESET1_ARG);
        let mut ret = lf_checker_rt::callee_thiscall!(RESET2_CALLEE, u32, this, RESET2_ARG);
        let mut i = 0u32;
        while (i as i32) < (((this + COUNT) as *const i32).read_unaligned()) {
            ret = lf_checker_rt::callee_thiscall!(ENTRY_CALLEE, u32, this, i);
            i = i.wrapping_add(1);
            if !((i as i32) < (((this + COUNT) as *const i32).read_unaligned())) {
                break;
            }
        }
        ret
    }
});
