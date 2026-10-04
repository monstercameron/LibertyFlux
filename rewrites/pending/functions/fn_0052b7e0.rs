// original: 0x0052b7e0 lb_race17standard_vf12
/// Map an index through the aux table, then find its key row, or -1.
///
/// Fetches the player_schema::Leaderboard_Ranked_Race17Standard tables (id 16), reads `aux[idx]`, and linearly
/// scans the key column for that value, returning the row index.
/// Returns -1 when the fetch fails, the aux entry is -1, the key
/// table is empty, or the value is absent (the count bound is
/// unsigned). Stdcall/1; entry registers are ignored.
export!(stdcall, rw_0052b7e0(idx: u32) -> i32 {
    unsafe {
        let mut info = [0u32; 6];
        let ok = callee_fastcall!(0, u32, 16, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return -1;
        }
        let aux = info[5];
        let v = *((aux.wrapping_add(idx.wrapping_mul(4))) as *const u32);
        if v == 0xffffffff {
            return -1;
        }
        let count = info[1];
        if count == 0 {
            return -1;
        }
        let keys = info[2];
        let mut i = 0u32;
        while i < count {
            let k = *((keys.wrapping_add(i.wrapping_mul(4))) as *const u32);
            if k == v {
                return i as i32;
            }
            i = i.wrapping_add(1);
        }
        -1
    }
});
