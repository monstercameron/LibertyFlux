// original: 0x005667c0 race90_vf12_row_key_position

/// Position of a leaderboard row's key inside the column key list.
///
/// `index` selects a row of the leaderboard identified by `LEADERBOARD_ID`.
/// The loader callee fills `info` (entry count at `+0x04`, key-list pointer
/// at `+0x08`, row-key-table pointer at `+0x14`). When the loader reports
/// failure, the row's key is missing (`NOT_FOUND`), or the list is empty,
/// the result is `NOT_FOUND`. Otherwise the key list is scanned with an
/// unsigned bound and the first matching position is returned.
///
/// Edge cases: an empty list returns `NOT_FOUND` without reading the list;
/// a key that only occurs at or past the bound is not found.
///
/// Original: thiscall, one stack word; incoming ECX is ignored.
lf_checker_rt::export!(thiscall, rw_005667c0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x125;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const INFO_COUNT: usize = 1; // +0x04: number of keys
        const INFO_KEYS: usize = 2; // +0x08: key list
        const INFO_ROWS: usize = 5; // +0x14: row-key table
        const CALLEE_FILL: u32 = 1;

        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_FILL, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let rows = info[INFO_ROWS];
        let key = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = info[INFO_KEYS];
        let mut i = 0u32;
        while i < count {
            let k = (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if k == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
