// original: 0x009321D0 imposter_cache_slot_lookup

/// Search the active 64-byte cache record for `key`. The active record is
/// selected by the global mode, and its sixteen word entries are scanned in
/// order. A hit sets the corresponding bit in that record's used mask and
/// returns the slot index. A miss calls the scripted rate helper, conditionally
/// updates the previous-key globals when the count divides the configured
/// denominator, and returns `u32::MAX`.
///
/// The original unsigned divisor comparison is `sample <= 1`; samples above
/// one are incremented before the unsigned remainder check. The direct helper
/// call is intercepted by the checker contract.
lf_checker_rt::export!(cdecl, rw_009321D0(key: u32) -> u32 {
    const MODE_VA: u32 = 0x011A_2A60;
    const TABLE_VA: u32 = 0x011A_29E0;
    const USED_MASK_VA: u32 = 0x011A_29D8;
    const DENOMINATOR_VA: u32 = 0x011A_2A64;
    const PREVIOUS_KEY_VA: u32 = 0x0103_6BD4;
    const HAVE_PREVIOUS_VA: u32 = 0x011A_2954;
    const SLOT_COUNT: u32 = 16;
    const SLOT_STRIDE: u32 = 4;
    const RECORD_STRIDE: u32 = 64;
    const RATE_HELPER_ID: u32 = 1;

    let mode = unsafe { lf_checker_rt::global::<u32>(MODE_VA).read_unaligned() };
    let active_record = 1u32.wrapping_sub(mode);
    let table = lf_checker_rt::relocated(TABLE_VA)
        .wrapping_add(active_record.wrapping_mul(RECORD_STRIDE));

    for slot_index in 0..SLOT_COUNT {
        let address = table.wrapping_add(slot_index * SLOT_STRIDE);
        let stored_key = unsafe { (address as *const u32).read_unaligned() };
        if stored_key == key {
            let mask_address = lf_checker_rt::relocated(USED_MASK_VA)
                .wrapping_add(active_record.wrapping_mul(SLOT_STRIDE));
            let old_mask = unsafe { (mask_address as *const u32).read_unaligned() };
            let slot_bit = 1u32.wrapping_shl(slot_index);
            unsafe { (mask_address as *mut u32).write_unaligned(old_mask | slot_bit) };
            return slot_index;
        }
    }

    let sample = lf_checker_rt::callee_cdecl!(RATE_HELPER_ID, u32,);
    let divisor = if sample <= 1 { sample } else { sample.wrapping_add(1) };
    let denominator = unsafe { lf_checker_rt::global::<u32>(DENOMINATOR_VA).read_unaligned() };
    if denominator % divisor == 0 {
        let previous = unsafe { lf_checker_rt::global::<u32>(PREVIOUS_KEY_VA).read_unaligned() };
        let next_previous = if previous == u32::MAX { key } else { previous };
        unsafe {
            lf_checker_rt::global::<u32>(PREVIOUS_KEY_VA).write_unaligned(next_previous);
            lf_checker_rt::global::<u8>(HAVE_PREVIOUS_VA).write(1);
        }
    }

    u32::MAX
});
