// original: 0x00e5d170 timer_table_fill_ones
/// Fill the 256-byte timer table at `0x01908CA0` with all-ones words.
///
/// Writes two `0xFFFFFFFF` dwords at the table head, then copies 62 dwords
/// forward from the head to 8 bytes past it. The ranges overlap, so the head
/// words propagate through the whole table (matches `rep movsd` with the
/// direction flag clear). Preserves ESI/EDI; no meaningful return value.
export!(cdecl, rw_00e5d170() -> u32 {
    unsafe {
        *global::<u32>(0x1908CA0) = 0xFFFFFFFF;
        *global::<u32>(0x1908CA4) = 0xFFFFFFFF;
        let src = global::<u32>(0x1908CA0);
        let dst = global::<u32>(0x1908CA8);
        for i in 0..62 {
            *dst.add(i) = *src.add(i);
        }
        0
    }
});
