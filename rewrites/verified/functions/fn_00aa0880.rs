// original: 0x00aa0880 stream_slot_mark_live (proposed)

/// Set bits 0 and 3 of a slot's flag word and return the slot pointer.
///
/// `obj` points to a record with its flag word at `+0x254`. Bits 0 (live)
/// and 3 are set; the return value is `obj` itself, unchanged.
///
/// Original: 0x00aa0880 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00aa0880(obj: u32) -> u32 {
    unsafe {
        const FLAGS_OFF: u32 = 0x254;
        const MARK_BITS: u32 = (1 << 3) | 1;
        let slot = (obj + FLAGS_OFF) as *mut u32;
        slot.write_unaligned(slot.read_unaligned() | MARK_BITS);
        obj
    }
});
