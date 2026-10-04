// original: 0x005438A0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_7, player_schema::LeaderboardInfo, 10>::vf13
/// Mapped value for a key looked up in the board's key column.
/// ///
/// /// Calls the leaderboard-info callee (fastcall slot 0) with this board's
/// /// numeric id in ECX and a scratch info block in EDX. When the callee
/// /// reports failure, or the signed `count` (at +0x0c) is not positive, the
/// /// result is NOT_FOUND. Otherwise the key array (at +0x10) is scanned
/// /// linearly for `key`; on a match at position i the mapping array (at
/// /// +0x14) entry i is returned. No match gives NOT_FOUND. The object
/// /// pointer in ECX is unused. Original is thiscall with one stack word;
/// /// its re-check of the found index against -1 is dead (the index is
/// /// always in range) and is not reproduced.
lf_checker_rt::export!(thiscall, rw_005438a0(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADER_ID: u32 = 0xA8;
        const INFO_COUNT: u32 = 0x0C;
        const INFO_KEYS: u32 = 0x10;
        const INFO_MAPPED: u32 = 0x14;
        const INFO_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADER_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = info[(INFO_COUNT / 4) as usize] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = info[(INFO_KEYS / 4) as usize] as *const u32;
        let mapped = info[(INFO_MAPPED / 4) as usize] as *const u32;
        let mut i = 0i32;
        loop {
            if keys.add(i as usize).read_unaligned() == key {
                return mapped.add(i as usize).read_unaligned();
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
