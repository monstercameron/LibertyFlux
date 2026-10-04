// original: 0x0052c100 lb_race19standard_vf13
/// Look up a leaderboard value by key, or -1 when missing.
///
/// Fetches the player_schema::Leaderboard_Ranked_Race19Standard column table (id 19), linearly scans its key
/// column for `key`, and returns the paired value column entry.
/// Returns -1 when the fetch fails, the table is empty, or the key
/// is absent. Stdcall/1; entry registers are ignored.
export!(stdcall, rw_0052c100(key: u32) -> i32 {
    unsafe {
        let mut info = [0u32; 6];
        let ok = callee_fastcall!(0, u32, 19, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return -1;
        }
        let count = info[3] as i32;
        if count <= 0 {
            return -1;
        }
        let keys = info[4];
        let vals = info[5];
        let mut i = 0i32;
        while i < count {
            let k = *((keys.wrapping_add((i as u32).wrapping_mul(4))) as *const u32);
            if k == key {
                return *((vals.wrapping_add((i as u32).wrapping_mul(4))) as *const i32);
            }
            i += 1;
        }
        -1
    }
});
