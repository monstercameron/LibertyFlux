// original: 0x0052bef0 lb_race18standard_vf7
/// Read one leaderboard table entry by index, or -1 on fetch failure.
///
/// Fetches the player_schema::Leaderboard_Ranked_Race18Standard column table (id 18) and returns entry `idx`.
/// Only the fetch-failure path returns -1; the index itself is not
/// bounds-checked. Stdcall/1; entry registers are ignored.
export!(stdcall, rw_0052bef0(idx: u32) -> i32 {
    unsafe {
        let mut info = [0u32; 6];
        let ok = callee_fastcall!(0, u32, 18, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return -1;
        }
        let table = info[4];
        *((table.wrapping_add(idx.wrapping_mul(4))) as *const i32)
    }
});
