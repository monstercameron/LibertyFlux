// original: 0x009545B0 tagged_table_find (proposed)

/// Find an object id in a table of tagged pointers.
///
/// Scans `COUNT` (0x200) 12-byte entries at `TABLE` (index compared
/// SIGNED against the bound, always 512 iterations). Skips null entries;
/// for the rest reads a tag byte at object+0 and accepts only `TAG_A`
/// (0x1B) or `TAG_B` (0x1C); returns 1 when such an object's dword id at
/// `ID_OFF` (0xC) equals `want`, else 0. Original is cdecl/1, returns AL.
lf_checker_rt::export!(cdecl, rw_009545B0(want: u32) -> u32 {
    const TABLE: u32 = 0x011FF074;
    const COUNT: i16 = 0x200;
    const ENTRY_WORDS: u32 = 3;
    const TAG_A: u8 = 0x1B;
    const TAG_B: u8 = 0x1C;
    const ID_OFF: u32 = 0xC;
    let mut i: i16 = 0;
    while i < COUNT {
        let slot = lf_checker_rt::relocated(TABLE)
            .wrapping_add((i as u32).wrapping_mul(ENTRY_WORDS).wrapping_mul(4));
        let obj = unsafe { (slot as *const u32).read_unaligned() };
        if obj != 0 {
            let tag = unsafe { (obj as *const u8).read() };
            if tag == TAG_A || tag == TAG_B {
                let id = unsafe { (obj.wrapping_add(ID_OFF) as *const u32).read_unaligned() };
                if id == want {
                    return 1;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    0
});
