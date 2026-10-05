// original: 0x00c809b0 conv_table_name (proposed)

/// Name of conversation row `id`, or the unknown-name string when absent.
///
/// Scans the table at `0x104b990` (stride `0xb0`, id at `+0`, `-1` ends the
/// scan) for `id` and returns the pointer kept at the row's `+4`. With no
/// match, returns a pointer to the shared unknown-name string.
///
/// Original: cdecl, one stack word (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c809b0(id: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x104b990;
        const STRIDE: u32 = 0xb0;
        const NAME_OFF: u32 = 4;
        const END_ID: u32 = 0xffff_ffff;
        const UNKNOWN: u32 = 0xed4f28;
        let base = lf_checker_rt::relocated(TABLE);
        let mut i: u32 = 0;
        loop {
            let ent = base.wrapping_add(i.wrapping_mul(STRIDE));
            let cur = (ent as *const u32).read_unaligned();
            if cur == END_ID {
                return lf_checker_rt::relocated(UNKNOWN);
            }
            if cur == id {
                return ((ent + NAME_OFF) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
        }
    }
});
