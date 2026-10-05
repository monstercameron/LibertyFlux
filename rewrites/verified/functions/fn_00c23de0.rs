// original: 0x00c23de0 stream_row_lookup (proposed)
/// Look up `key`. Keys above 0xe0f scan the row table at 0x016b9c30 (0x14
/// bytes per row, `count` rows from 0x016b9d70, signed): the row whose word
/// at +0xc equals the key yields the word at +8, else 0. Keys at or below
/// 0xe0f index the small-key array directly: word at
/// `*(0x016b8f84) + key*12 + 4`.
///
/// Original: 0x00c23de0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c23de0(key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x016b9d70;
        const TABLE: u32 = 0x016b9c30;
        const SMALL_BASE_PTR: u32 = 0x016b8f84;
        const ROW: u32 = 0x14;
        const KEY_OFF: u32 = 0x0c;
        const VAL_OFF: u32 = 0x08;
        const SMALL_MAX: u32 = 0xe0f;
        if key > SMALL_MAX {
            let n = (lf_checker_rt::relocated(COUNT) as *const i32).read_unaligned();
            if n <= 0 {
                return 0;
            }
            let mut p = lf_checker_rt::relocated(TABLE);
            let mut i: i32 = 0;
            loop {
                if (p.wrapping_add(KEY_OFF) as *const u32).read_unaligned() == key {
                    return (p.wrapping_add(VAL_OFF) as *const u32).read_unaligned();
                }
                i = i.wrapping_add(1);
                p = p.wrapping_add(ROW);
                if !(i < n) {
                    return 0;
                }
            }
        } else {
            let arr = (lf_checker_rt::relocated(SMALL_BASE_PTR) as *const u32).read_unaligned();
            let off = key.wrapping_mul(3).wrapping_add(1).wrapping_mul(4);
            (arr.wrapping_add(off) as *const u32).read_unaligned()
        }
    }
});
