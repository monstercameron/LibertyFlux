// original: 0x008BEA60 ui_slot_write_pair (proposed)

/// Fetch a float through a helper, announce it, then write an input slot and
/// report it through the table handler.
///
/// The six stack words are an unread word, the slot index, a pointer to two
/// words, a second pointer to two words, a float word and a second unread
/// word. First a helper callee is asked (through a frame slot) for the
/// address of a float; the float is kept in the frame and the consumer callee
/// is told (2, 0, frame-slot, 0). Then slot `idx` (24 bytes at the slot
/// table plus `idx * 24`, the value word at +0 deliberately NOT written)
/// receives the two pointed-to words at +4, the other two at +12 and the
/// float bits at +20, and the consumer is told (2, slot+4, slot+12, 0).
/// Finally the table handler for `idx` runs with (table[idx], slot+4, float)
/// and its answer is returned (cdecl, six stack words; the first and last
/// are not read).
lf_checker_rt::export!(cdecl, rw_008BEA60(
    _gap0: u32,
    idx: u32,
    p1: u32,
    p2: u32,
    float_bits: u32,
    _gap5: u32,
) -> u32 {
    unsafe {
        /// First byte of the slot table and bytes per slot.
        const SLOT_TABLE: u32 = 0x01161850;
        const SLOT_STRIDE: u32 = 24;
        /// Offsets of the fields inside a slot.
        const OFF_FIRST_PAIR: u32 = 0x04;
        const OFF_SECOND_PAIR: u32 = 0x0C;
        const OFF_FLOAT: u32 = 0x14;
        /// Global table of handler addresses, indexed like the slots.
        const HANDLER_TABLE: u32 = 0x01160C0C;
        /// Callee ids in the contract: float helper, consumer, handler.
        const HELPER: u32 = 1;
        const CONSUMER: u32 = 2;
        const HANDLER: u32 = 3;
        let rd32 = |a: u32| (a as *const u32).read_unaligned();
        let wr32 = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        // Frame slot shared with the helper and the first consumer call
        // (its address is skipped in the call comparison; a snapshot of
        // these words observes the contents instead).
        let mut fslot = [0u32; 4];
        let fslot_ptr = fslot.as_mut_ptr() as u32;
        let float_at = lf_checker_rt::callee_stdcall!(HELPER, u32, fslot_ptr);
        let fetched = rd32(float_at);
        fslot[0] = fetched;
        fslot[1] = 0;
        lf_checker_rt::callee_cdecl!(CONSUMER, u32, 2u32, 0u32, fslot_ptr, 0u32);
        let slot = SLOT_TABLE + idx.wrapping_mul(SLOT_STRIDE);
        let pair1 = lf_checker_rt::relocated(slot + OFF_FIRST_PAIR);
        wr32(pair1, rd32(p1));
        wr32(pair1 + 4, rd32(p1 + 4));
        let pair2 = lf_checker_rt::relocated(slot + OFF_SECOND_PAIR);
        wr32(pair2, rd32(p2));
        wr32(pair2 + 4, rd32(p2 + 4));
        wr32(lf_checker_rt::relocated(slot + OFF_FLOAT), float_bits);
        lf_checker_rt::callee_cdecl!(CONSUMER, u32, 2u32, pair1, pair2, 0u32);
        let entry = (lf_checker_rt::relocated(
            HANDLER_TABLE.wrapping_add(idx.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        lf_checker_rt::callee_cdecl!(HANDLER, u32, entry, pair1, fetched)
    }
});
