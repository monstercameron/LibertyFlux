// original: 0x00a82e10 TaskList_FindBySeqAndStage

/// Find the first linked-list entry matching a sequence and requested stage.
///
/// `this` begins with the head pointer. Entries are linked by the word at
/// `+0x0C`, store the sequence at `+0x04`, and pack the stage in bits 1..3 of
/// the word at `+0x08`. The sequence and stage arguments are unsigned and
/// compared for exact equality. Stage 5 is a wildcard; otherwise the packed
/// stage must equal the requested value. Returns the matching entry or null.
lf_checker_rt::export!(thiscall, rw_00a82e10(this: u32, sequence_id: u32, requested_stage: u32) -> u32 {
    const LIST_HEAD: u32 = 0x00;
    const ENTRY_SEQUENCE: u32 = 0x04;
    const ENTRY_PACKED_STAGE: u32 = 0x08;
    const ENTRY_NEXT: u32 = 0x0C;
    const STAGE_SHIFT: u32 = 1;
    const STAGE_MASK: u32 = 0x07;
    const WILDCARD_STAGE: u32 = 5;

    let mut entry = unsafe { ((this + LIST_HEAD) as *const u32).read_unaligned() };
    while entry != 0 {
        let entry_sequence = unsafe { ((entry + ENTRY_SEQUENCE) as *const u32).read_unaligned() };
        if entry_sequence == sequence_id {
            if requested_stage == WILDCARD_STAGE {
                return entry;
            }
            let packed = unsafe { ((entry + ENTRY_PACKED_STAGE) as *const u32).read_unaligned() };
            let entry_stage = (packed >> STAGE_SHIFT) & STAGE_MASK;
            if entry_stage == requested_stage {
                return entry;
            }
        }
        entry = unsafe { ((entry + ENTRY_NEXT) as *const u32).read_unaligned() };
    }
    0
});
