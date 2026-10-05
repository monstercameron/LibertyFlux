// original: 0x008F9910 stream_slot_ready_check
/// Test whether streaming slot `idx` is populated and flagged ready.
///
/// The object holds an entry array at `+0` (0x54-byte entries) and a
/// count at `+4`. Returns 1 only when the array is non-null, `idx` is
/// a signed in-range index, the entry's first word is nonzero and its
/// flag byte at `+0x50` is nonzero; otherwise 0. Thiscall, one stack
/// argument, boolean in al.
export!(thiscall, rw_008f9910(this: u32, idx: u32) -> u32 {
    unsafe {
        const ENTRY_STRIDE: u32 = 0x54;
        const READY_FLAG: u32 = 0x50;
        let base = (this as *const u32).read_unaligned();
        if base == 0 {
            return 0;
        }
        if (idx as i32) < 0 {
            return 0;
        }
        let count = ((this + 4) as *const u32).read_unaligned();
        if (idx as i32) >= (count as i32) {
            return 0;
        }
        let e = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        if (e as *const u32).read_unaligned() == 0 {
            return 0;
        }
        if ((e + READY_FLAG) as *const u8).read() == 0 {
            return 0;
        }
        1
    }
});
