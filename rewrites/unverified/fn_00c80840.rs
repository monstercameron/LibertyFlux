// original: 0x00c80840 conv_table_lookup (proposed)

/// Look `id` up in the conversation table and report its two payload words.
///
/// Scans the table at `0x104b990` (stride `0xb0`, id at `+0`, `-1` ends the
/// scan) for `id`. On a match whose word at `+0x18` is positive, stores the
/// words at `+0x1c` and `+0x18` through `out1` and `out2` and returns 1. A
/// match with a non-positive `+0x18`, or no match at all, returns 0. Only AL
/// carries the result.
///
/// Original: cdecl, three stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c80840(id: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x104b990;
        const STRIDE: u32 = 0xb0;
        const ID_OFF: u32 = 0;
        const COUNT_OFF: u32 = 0x18;
        const VALUE_OFF: u32 = 0x1c;
        const END_ID: u32 = 0xffff_ffff;
        let base = lf_checker_rt::relocated(TABLE);
        let mut i: u32 = 0;
        loop {
            let ent = base.wrapping_add(i.wrapping_mul(STRIDE));
            let cur = ((ent + ID_OFF) as *const u32).read_unaligned();
            if cur == END_ID {
                return 0;
            }
            if cur == id {
                break;
            }
            i = i.wrapping_add(1);
        }
        let ent = base.wrapping_add(i.wrapping_mul(STRIDE));
        let count = ((ent + COUNT_OFF) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let value = ((ent + VALUE_OFF) as *const u32).read_unaligned();
        (out1 as *mut u32).write_unaligned(value);
        (out2 as *mut u32).write_unaligned(count as u32);
        1
    }
});
