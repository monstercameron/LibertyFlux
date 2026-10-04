// original: 0x0055f6a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_64, player_schema::LeaderboardInfo, 10>::vf13

/// Maps a key to its value through this board's parallel tables.
/// ///
/// The loader callee (id `LEADERBOARD_ID`) fills three out words through a
/// frame pointer: the entry count (at `+0x0c`), the key array (at `+0x10`)
/// and the value array (at `+0x14`). When the loader reports failure, or
/// the count is zero or negative, the result is -1. Otherwise the keys
/// are scanned with a signed bound for `key`; the result is the value at
/// the matching position, or -1 when no key matches. (The original has a
/// dead compare of the found index against -1 left over from signed
/// codegen; the index is never negative there.)
/// ///
/// Original: 0x0055f6a0 (stdcall, one stack word). Episodic race board 64;
/// only the loader id differs between instantiations.
lf_checker_rt::export!(stdcall, rw_0055f6a0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10b;
        const OUT_COUNT: usize = 3;
        const OUT_KEYS: usize = 4;
        const OUT_VALUES: usize = 5;
        const MISSING: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return MISSING;
        }
        let count = out[OUT_COUNT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = out[OUT_KEYS] as *const u32;
        let values = out[OUT_VALUES] as *const u32;
        let mut i = 0i32;
        loop {
            if keys.add(i as usize).read_unaligned() == key {
                return values.add(i as usize).read_unaligned();
            }
            i += 1;
            if i >= count {
                return MISSING;
            }
        }
    }

});
