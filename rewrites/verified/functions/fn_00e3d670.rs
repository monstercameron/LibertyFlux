// original: 0x00e3d670 StatList_FindFirst
// 0x00E3D670: find the first list entry accepted by a predicate. (thiscall/0)
//
// Resolves the entry table through the group lookup, then scans 8-byte
// records until the end marker, asking the predicate about each index.
// Returns the accepted index, or zero when nothing is accepted.
export!(thiscall, rw_00e3d670(this: u32) -> u32 {
    unsafe {
        const END_MARK: u16 = 0xFF9D;
        const STRIDE: u32 = 8;
        let key = *(this as *const u32).add(1);
        let table = callee_cdecl!(1, u32, key);
        if table == 0 {
            return 0;
        }
        let mut idx = 0u32;
        if *(table as *const u16) == END_MARK {
            return 0;
        }
        loop {
            let accepted = callee_cdecl!(2, u32, key, idx, 0);
            if (accepted as u8) != 0 {
                return idx;
            }
            idx += 1;
            if *((table.wrapping_add(idx.wrapping_mul(STRIDE))) as *const u16) == END_MARK {
                return 0;
            }
        }
    }
});
