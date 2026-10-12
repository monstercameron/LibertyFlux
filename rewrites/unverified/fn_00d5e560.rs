// original: 0x00D5E560 streaming_pick_random_effect_entry

/// Select an entry using the entity's 64-bit weighted-choice state.
///
/// `this` addresses 12-byte choice records after an initial eight-byte header.
/// The two stack arguments select a record as `row * 42 + column`. A record's
/// base index is at offset 8 and its signed divisor is at offset 12. A base
/// index of `u32::MAX` returns null without changing the choice state. Otherwise
/// the function advances the two-word state at the named global locations by
/// multiplying the low word by the fixed multiplier and adding the high word,
/// masks the result to a nonnegative signed dividend, computes the signed
/// remainder, and returns an entry pointer in the array at offset `0x52b8`.
/// The method is thiscall, cleans its two stack arguments, and returns a pointer
/// into the same object.
lf_checker_rt::export!(thiscall, rw_streaming_pick_random_effect_entry(
    this_ptr: u32,
    row: u32,
    column: u32
) -> u32 {
    const CHOICES_PER_ROW: u32 = 42;
    const RECORD_STRIDE: u32 = 12;
    const RECORD_TABLE_OFFSET: u32 = 0;
    const RECORD_BASE_INDEX_OFFSET: u32 = 8;
    const RECORD_DIVISOR_OFFSET: u32 = 12;
    const RANDOM_STATE_LOW_VA: u32 = 0x011101A0;
    const RANDOM_STATE_HIGH_VA: u32 = 0x011101A4;
    const RANDOM_MULTIPLIER: u64 = 0x5CDCFAA7;
    const ENTRY_ARRAY_OFFSET: u32 = 0x52B8;
    const ENTRY_STRIDE: u32 = 0x7C;
    const NO_ENTRY: u32 = u32::MAX;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let record_index = row.wrapping_mul(CHOICES_PER_ROW).wrapping_add(column);
    let record = this_ptr
        .wrapping_add(RECORD_TABLE_OFFSET)
        .wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
    let base_index = unsafe { read_u32(record.wrapping_add(RECORD_BASE_INDEX_OFFSET)) };
    if base_index == NO_ENTRY {
        return 0;
    }

    let divisor = unsafe { read_u32(record.wrapping_add(RECORD_DIVISOR_OFFSET)) as i32 };
    let low_state = unsafe { lf_checker_rt::global::<u32>(RANDOM_STATE_LOW_VA).read_unaligned() };
    let high_state = unsafe { lf_checker_rt::global::<u32>(RANDOM_STATE_HIGH_VA).read_unaligned() };
    let advanced_state = (u64::from(low_state))
        .wrapping_mul(RANDOM_MULTIPLIER)
        .wrapping_add(u64::from(high_state));
    unsafe {
        write_u32(
            lf_checker_rt::global::<u32>(RANDOM_STATE_LOW_VA) as u32,
            advanced_state as u32,
        );
        write_u32(
            lf_checker_rt::global::<u32>(RANDOM_STATE_HIGH_VA) as u32,
            (advanced_state >> 32) as u32,
        );
    }

    let dividend = (advanced_state as u32 & 0x7FFF_FFFF) as i32;
    let remainder = dividend % divisor;
    let entry_index = base_index.wrapping_add(remainder as u32);
    this_ptr
        .wrapping_add(ENTRY_ARRAY_OFFSET)
        .wrapping_add(entry_index.wrapping_mul(ENTRY_STRIDE))
});
