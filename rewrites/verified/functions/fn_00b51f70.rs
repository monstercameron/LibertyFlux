// original: 0x00B51F70 euphoria_reset_all_slots

/// Reset three adjacent slot records by calling the slot rebind routine for
/// each one. The records are spaced `0x520` bytes apart; each call passes the
/// record in ECX and a zero stack argument. The last callee's EAX value is
/// returned, matching the original's final call result.
lf_checker_rt::export!(thiscall, rw_b51f70(this: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0x520;
    let mut last_result = 0;
    for slot_index in 0..3 {
        let slot = this.wrapping_add(slot_index * SLOT_STRIDE);
        last_result = lf_checker_rt::callee_thiscall!(1, u32, slot, 0);
    }
    last_result
});
