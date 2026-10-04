// original: 0x005433D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_6, player_schema::LeaderboardInfo, 10>::vf12
/// Position of a row's value inside the board's key column.
/// ///
/// /// Calls the leaderboard-info callee (fastcall slot 0) with this board's
/// /// numeric id in ECX and a scratch info block in EDX. When the callee
/// /// reports failure the result is NOT_FOUND. Otherwise the row's value is
/// /// read from the value array (at +0x14) at `index`; a value of NOT_FOUND
/// /// ends the search. The key array (at +0x08) is then scanned linearly
/// /// for the first of `count` (at +0x04, compared unsigned) entries equal
/// /// to the value, and its position returned, or NOT_FOUND when absent.
/// /// The object pointer in ECX is unused. Original is thiscall with one
/// /// stack word.
lf_checker_rt::export!(thiscall, rw_005433d0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADER_ID: u32 = 0xA6;
        const INFO_COUNT: u32 = 0x04;
        const INFO_KEYS: u32 = 0x08;
        const INFO_VALUES: u32 = 0x14;
        const INFO_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADER_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let values = info[(INFO_VALUES / 4) as usize];
        let val = (values as *const u32).add(index as usize).read_unaligned();
        if val == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[(INFO_COUNT / 4) as usize];
        let keys = info[(INFO_KEYS / 4) as usize] as *const u32;
        let mut i = 0u32;
        while i < count {
            if keys.add(i as usize).read_unaligned() == val {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
