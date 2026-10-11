// original: 0x00A82FD0 Sequence_FindById

/// Find a sequence entry whose identifier and packed stage both match.
///
/// `this` begins with the list head. Each entry stores its identifier at
/// `+0x04`, its stage in bits 1..3 of `+0x08`, and its next pointer at
/// `+0x0C`. Both stack arguments are unsigned. On success the function sets
/// the low byte of the matching entry pointer to one and returns that value;
/// on failure it returns zero.
lf_checker_rt::export!(thiscall, rw_00a82fd0(this: u32, sequence_id: u32, requested_stage: u32) -> u32 {
    const ENTRY_SEQUENCE: u32 = 0x04;
    const ENTRY_PACKED_STAGE: u32 = 0x08;
    const ENTRY_NEXT: u32 = 0x0C;
    const STAGE_SHIFT: u32 = 1;
    const STAGE_MASK: u32 = 0x07;
    const RETURN_LOW_BYTE_MASK: u32 = 0xFFFF_FF00;

    let mut entry = unsafe { (this as *const u32).read_unaligned() };
    while entry != 0 {
        let entry_sequence = unsafe { ((entry + ENTRY_SEQUENCE) as *const u32).read_unaligned() };
        if entry_sequence == sequence_id {
            let packed = unsafe { ((entry + ENTRY_PACKED_STAGE) as *const u32).read_unaligned() };
            if ((packed >> STAGE_SHIFT) & STAGE_MASK) == requested_stage {
                return (entry & RETURN_LOW_BYTE_MASK) | 1;
            }
        }
        entry = unsafe { ((entry + ENTRY_NEXT) as *const u32).read_unaligned() };
    }
    0
});
