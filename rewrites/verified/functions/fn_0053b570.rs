// original: 0x0053B570 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopSwatAssault_BG_TIME, player_schema::LeaderboardInfo, 10>::vf13

/// Map a leaderboard row key to its stored value.
///
/// Queries table TABLE_ID; the info descriptor's word 3 is the signed row
/// count, word 4 points at the row-key array, word 5 at the parallel value
/// array. Returns the value whose key equals the argument, or NOT_FOUND (-1)
/// when the query fails, the count is zero or negative, or no key matches.
/// The count comparison is signed, the scan linear.
///
/// Original: thiscall with one stack word; ECX (this) is unused.

lf_checker_rt::export!(thiscall, rw_0053b570(this: u32, value: u32) -> u32 {
    unsafe {
        const TABLE_ID: u32 = 0x15;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, TABLE_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = info[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let items = info[4] as *const u32;
        let values = info[5] as *const u32;
        let mut i = 0i32;
        loop {
            if items.wrapping_add(i as usize).read_unaligned() == value {
                return values.wrapping_add(i as usize).read_unaligned();
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
