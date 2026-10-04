// original: 0x0052bad0 lb_race17standard_vf8
/// Classify a leaderboard row's lookup result as 4, 8, or 0.
///
/// Fetches the player_schema::Leaderboard_Ranked_Race17Standard column table (id 16), passes entry `idx`
/// through a second lookup, and maps 1..=5 to [4, 8, 8, 0, 4] (anything
/// else, including fetch failure, yields 0). Stdcall/1; entry
/// registers are ignored.
export!(stdcall, rw_0052bad0(idx: u32) -> u32 {
    unsafe {
        let mut info = [0u32; 6];
        let ok = callee_fastcall!(0, u32, 16, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let table = info[5];
        let key = *((table.wrapping_add(idx.wrapping_mul(4))) as *const u32);
        let r = callee_thiscall!(1, u32, key);
        if r == 0xffffffff {
            return 0;
        }
        let d = r.wrapping_sub(1);
        if d > 4 {
            return 0;
        }
        match d + 1 {
            1 => 4,
            2 => 8,
            3 => 8,
            4 => 0,
            5 => 4,
            _ => 0,
        }
    }
});
