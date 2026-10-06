// original: 0x008F6EF0 Input_ResetSlots

/// Call the per-slot reset with argument 1 for each of the four slots from
/// `SLOT_FIRST` up to (signed) `SLOT_LAST` in steps of `SLOT_STRIDE`.
/// Returns the last reset's answer. Convention: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008f6ef0() -> u32 {
    unsafe {
        const SLOT_FIRST: u32 = 0x0118D470;
        const SLOT_LAST: u32 = 0x0118D760;
        const SLOT_STRIDE: u32 = 0xBC;
        const RESET: u32 = 1;
        let bound = lf_checker_rt::relocated(SLOT_LAST);
        let mut slot = lf_checker_rt::relocated(SLOT_FIRST);
        let mut r: u32 = 0;
        loop {
            r = lf_checker_rt::callee_thiscall!(RESET, u32, slot, 1);
            slot = slot.wrapping_add(SLOT_STRIDE);
            if !((slot as i32) < (bound as i32)) {
                break;
            }
        }
        r
    }
});
