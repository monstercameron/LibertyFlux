// original: 0x008BEA00 ui_slot_write (proposed)

/// Write one 24-byte input slot and notify the slot consumer.
///
/// The six stack words are the slot index, a value word, a pointer to two
/// words, a second pointer to two words, a float word and an unread word.
/// Slot `idx` lives at the slot table plus `idx * 24`: the value word goes at
/// +0, the two pointed-to words at +4, the other two at +12, and the float
/// bits (copied, never interpreted) at +20. The consumer callee is then
/// called with (2, slot+4 pointer, slot+12 pointer, 0). Returns the callee's
/// answer (cdecl, six stack words; the sixth is not read).
lf_checker_rt::export!(cdecl, rw_008BEA00(
    idx: u32,
    value: u32,
    p1: u32,
    p2: u32,
    float_bits: u32,
    _gap: u32,
) -> u32 {
    unsafe {
        /// First byte of the slot table.
        const SLOT_TABLE: u32 = 0x01161850;
        /// Bytes per slot (three slots per index step of 8: idx*3*8).
        const SLOT_STRIDE: u32 = 24;
        /// Offsets of the fields inside a slot.
        const OFF_VALUE: u32 = 0x00;
        const OFF_FIRST_PAIR: u32 = 0x04;
        const OFF_SECOND_PAIR: u32 = 0x0C;
        const OFF_FLOAT: u32 = 0x14;
        /// Id of the slot-consumer callee in the contract.
        const CONSUMER: u32 = 1;
        let slot = SLOT_TABLE + idx.wrapping_mul(SLOT_STRIDE);
        let rd32 = |a: u32| (a as *const u32).read_unaligned();
        let wr32 = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        wr32(lf_checker_rt::relocated(slot + OFF_VALUE), value);
        let pair1 = lf_checker_rt::relocated(slot + OFF_FIRST_PAIR);
        wr32(pair1, rd32(p1));
        wr32(pair1 + 4, rd32(p1 + 4));
        let pair2 = lf_checker_rt::relocated(slot + OFF_SECOND_PAIR);
        wr32(pair2, rd32(p2));
        wr32(pair2 + 4, rd32(p2 + 4));
        wr32(lf_checker_rt::relocated(slot + OFF_FLOAT), float_bits);
        lf_checker_rt::callee_cdecl!(CONSUMER, u32, 2u32, pair1, pair2, 0u32)
    }
});
