// original: 0x00560a10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_68, player_schema::LeaderboardInfo, 10>::vf6

/// Finds a key's position in this board's key table.
/// 
/// The loader callee (id `LEADERBOARD_ID`) fills two out words through a
/// frame pointer: the entry count (at `+0x0c`) and the key array (at
/// `+0x10`). When the loader reports failure, or the count is zero or
/// negative, the result is -1. Otherwise the keys are scanned with a
/// signed bound for `key`; the result is the first matching index, or -1
/// when no key matches.
/// 
/// Original: 0x00560a10 (stdcall, one stack word). Episodic race board 68;
/// only the loader id differs between instantiations.
lf_checker_rt::export!(stdcall, rw_00560a10(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10f;
        const OUT_COUNT: usize = 3;
        const OUT_KEYS: usize = 4;
        const MISSING: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return MISSING;
        }
        let count = out[OUT_COUNT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = out[OUT_KEYS] as *const u32;
        let mut i = 0i32;
        loop {
            if keys.add(i as usize).read_unaligned() == key {
                return i as u32;
            }
            i += 1;
            if i >= count {
                return MISSING;
            }
        }
    }

});
