// original: 0x00c809f0 conv_table_tag (proposed)

/// Tag word of conversation row `id`, or -1 when absent.
///
/// Scans the table at `0x104b990` (stride `0xb0`, id at `+0`, `-1` ends the
/// scan) for `id` and returns the row's word at `+0x14`. With no match,
/// returns -1 (the scan's failing id, ORed with itself).
///
/// Original: cdecl, one stack word (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c809f0(id: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x104b990;
        const STRIDE: u32 = 0xb0;
        const TAG_OFF: u32 = 0x14;
        const END_ID: u32 = 0xffff_ffff;
        let base = lf_checker_rt::relocated(TABLE);
        let mut i: u32 = 0;
        loop {
            let ent = base.wrapping_add(i.wrapping_mul(STRIDE));
            let cur = (ent as *const u32).read_unaligned();
            if cur == END_ID {
                return END_ID;
            }
            if cur == id {
                return ((ent + TAG_OFF) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
        }
    }
});
