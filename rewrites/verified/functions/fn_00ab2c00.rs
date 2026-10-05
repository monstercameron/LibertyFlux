// original: 0x00ab2c00 stream_push_request (proposed)

/// Append a two-word request to the fixed queue, unless it is full.
///
/// The queue header holds its length at `+0x15105C` and the slots at
/// `+0x151060`, eight bytes each. When the length is already 128 or more
/// nothing happens; otherwise the two argument words land at slot `len`
/// and the length grows by one. No return value.
///
/// Original: 0x00ab2c00 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab2c00(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const LEN_OFF: u32 = 0x15105C;
        const SLOTS_OFF: u32 = 0x151060;
        const SLOT_BYTES: u32 = 8;
        const CAP: u32 = 0x80;
        let len = ((this + LEN_OFF) as *const u32).read_unaligned();
        if len < CAP {
            let slot = (this + SLOTS_OFF).wrapping_add(len.wrapping_mul(SLOT_BYTES));
            (slot as *mut u32).write_unaligned(a);
            // The original re-reads the length for the second store; it
            // cannot have changed, so the same slot is meant.
            let len2 = ((this + LEN_OFF) as *const u32).read_unaligned();
            let slot2 =
                (this + SLOTS_OFF + 4).wrapping_add(len2.wrapping_mul(SLOT_BYTES));
            (slot2 as *mut u32).write_unaligned(b);
            ((this + LEN_OFF) as *mut u32).write_unaligned(len.wrapping_add(1));
        }
        0
    }
});
