// original: 0x008FB780 HelpMsg_IsDisplayed
/// Test whether a help text matches the displayed slot.
///
/// Returns 0 unless the pending check passes. Then both the
/// candidate string and the slot text (at `+0x4b0`) are measured;
/// unequal lengths return 0, else the bounded comparison's boolean
/// result is returned. Thiscall, one stack argument, boolean in al.
export!(thiscall, rw_008fb780(this: u32, s: u32) -> u32 {
    unsafe {
        const SLOT_TEXT: u32 = 0x4b0;
        let t: u32 = callee_thiscall!(1, u32, this);
        if (t as u8) == 0 {
            return 0;
        }
        let l0: u32 = callee_cdecl!(2, u32, s);
        let slot = this.wrapping_add(SLOT_TEXT);
        let l1: u32 = callee_cdecl!(3, u32, slot);
        if (l0 & 0xFFFF) != (l1 & 0xFFFF) {
            return 0;
        }
        let r: u32 = callee_cdecl!(4, u32, s, slot, l0 & 0xFFFF);
        if (r as u8) != 0 { 1 } else { 0 }
    }
});
