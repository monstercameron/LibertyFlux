// original: 0x0052ba30 lb_race17standard_vf6
/// Find a leaderboard key's row index, or -1 when missing.
///
/// Fetches the player_schema::Leaderboard_Ranked_Race17Standard column table (id 16) and linearly scans its
/// key column for `key`, returning the first matching row index.
/// Returns -1 when the fetch fails, the table is empty, or the key
/// is absent. Stdcall/1; entry registers are ignored.
export!(stdcall, rw_0052ba30(key: u32) -> i32 {
    unsafe {
        let mut info = [0u32; 6];
        let ok = callee_fastcall!(0, u32, 16, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return -1;
        }
        let count = info[3] as i32;
        if count <= 0 {
            return -1;
        }
        let keys = info[4];
        let mut i = 0i32;
        while i < count {
            let k = *((keys.wrapping_add((i as u32).wrapping_mul(4))) as *const u32);
            if k == key {
                return i;
            }
            i += 1;
        }
        -1
    }
});
