// original: 0x00a9f040 stream_slot_clear_bit4 (proposed)

/// Clear bit 4 of a slot's flag word, then drop the slot's live bit when
/// neither of two state bits is set.
///
/// Same shape as the neighbouring bit-3 clearer: `obj` points to a record
/// with its flag word at `+0x254`. Bit 4 is cleared and written back. The
/// refreshed word is then examined: bit 3 selects the return of
/// `flags >> 3`, else bit 5 selects `flags >> 5`, else bit 0 (the live bit)
/// is cleared as well and the shifted value returned.
///
/// Original: 0x00a9f040 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a9f040(obj: u32) -> u32 {
    unsafe {
        const FLAGS_OFF: u32 = 0x254;
        const CLEARED_BIT: u32 = 1 << 4;
        const LIVE_BIT: u32 = 1;
        let slot = (obj + FLAGS_OFF) as *mut u32;
        slot.write_unaligned(slot.read_unaligned() & !CLEARED_BIT);
        let flags = slot.read_unaligned();
        let by_kind = flags >> 3;
        if by_kind & 1 != 0 {
            return by_kind;
        }
        let by_state = flags >> 5;
        if by_state & 1 != 0 {
            return by_state;
        }
        slot.write_unaligned(flags & !LIVE_BIT);
        by_state
    }
});
