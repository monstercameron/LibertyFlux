// original: 0x00e3ee30 StatRange_IsAllowed
// 0x00E3EE30: range gate: the record's level must cover its band span,
// and the band switch must be on. Returns 1 when open. (thiscall/0,
// AL-only: upper EAX keeps arithmetic leftovers)
export!(thiscall, rw_00e3ee30(this: *const u8) -> u32 {
    unsafe {
        let level = *(this.add(0x3cc) as *const i8);
        let mode = *(this.add(0x395));
        let enabled = if level > 1 { 0u32 } else { 1u32 };
        let lo = *(this.add(0x311)) as u32;
        let hi = *(this.add(0x313)) as u32;
        let extra = if mode != 0 { 1i32 } else { 0i32 };
        let span = (hi.wrapping_sub(lo) as i32).wrapping_add(extra);
        let threshold = (level as i32).wrapping_add(2);
        if threshold < span {
            0
        } else {
            enabled
        }
    }
});
