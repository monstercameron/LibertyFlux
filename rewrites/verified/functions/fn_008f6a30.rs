// original: 0x008F6A30 Input_CountActiveSlots

/// Call the per-slot probe for each of the four slots from `SLOT_FIRST` up
/// to (signed) `SLOT_LAST` in steps of `SLOT_STRIDE`, and return how many
/// answers had a nonzero low byte. Only AL is tested. Convention: cdecl, no
/// stack words. The slot addresses are relocated immediates.
lf_checker_rt::export!(cdecl, rw_008f6a30() -> u32 {
    unsafe {
        const SLOT_FIRST: u32 = 0x0118D470;
        const SLOT_LAST: u32 = 0x0118D760;
        const SLOT_STRIDE: u32 = 0xBC;
        const PROBE: u32 = 1;
        let bound = lf_checker_rt::relocated(SLOT_LAST);
        let mut slot = lf_checker_rt::relocated(SLOT_FIRST);
        let mut n: u32 = 0;
        loop {
            let r: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, slot);
            if r & 0xFF != 0 {
                n += 1;
            }
            slot = slot.wrapping_add(SLOT_STRIDE);
            if !((slot as i32) < (bound as i32)) {
                break;
            }
        }
        n
    }
});
