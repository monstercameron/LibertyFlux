// original: 0x00c292f0 audio_table_secondary_slot_is_valid

/// Returns whether the selected table slot has a nonnegative signed status.
///
/// `this_ptr` points to the object base and `slot_index` is the zero-based
/// slot number. The signed 32-bit status is at byte offset `STATUS_OFFSET`
/// plus `slot_index * SLOT_STRIDE`; negative statuses, including the all-ones
/// sentinel, return zero, while zero and positive statuses return one.
/// The 32-bit thiscall ABI places `this_ptr` in ECX and the index on the stack.
lf_checker_rt::export!(thiscall, rw_00c292f0(this_ptr: u32, slot_index: u32) -> u32 {
    const STATUS_OFFSET: u32 = 0x5c10;
    const SLOT_STRIDE: u32 = 8;
    let status_address = this_ptr
        .wrapping_add(STATUS_OFFSET)
        .wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
    let status = unsafe { (status_address as *const i32).read_unaligned() };
    u32::from(status >= 0)
});
