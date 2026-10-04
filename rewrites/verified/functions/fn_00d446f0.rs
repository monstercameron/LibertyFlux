// original: 0x00d446f0 ambient_table_find (proposed)

/// Find the first table entry whose tag word equals the key.
///
/// Reads the entry count (a 16-bit word) and the table base (an array of
/// object pointers) from their static slots. A zero count returns null
/// without touching the table. Otherwise each entry's object is read and its
/// first dword compared against `key`; the first match's object pointer is
/// returned, or null when nothing matches. The loop bound compare is signed,
/// matching the original, though the count is never negative.
///
/// Original: cdecl, one stack word (key), caller cleans up.
lf_checker_rt::export!(cdecl, rw_00d446f0(key: u32) -> u32 {
    unsafe {
        const TABLE_SLOT: u32 = 0x01720924;
        const COUNT_SLOT: u32 = 0x01720928;
        let n = (lf_checker_rt::global::<u16>(COUNT_SLOT) as *const u16).read_unaligned() as u32;
        if (n as i32) <= 0 {
            return 0;
        }
        let table =
            (lf_checker_rt::global::<u32>(TABLE_SLOT) as *const u32).read_unaligned();
        let mut i = 0u32;
        loop {
            let obj = ((table.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            let tag = (obj as *const u32).read_unaligned();
            if tag == key {
                return obj;
            }
            i = i.wrapping_add(1);
            if !((i as i32) < (n as i32)) {
                return 0;
            }
        }
    }
});
