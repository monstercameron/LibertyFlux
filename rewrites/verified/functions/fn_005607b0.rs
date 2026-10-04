// original: 0x005607b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_68, player_schema::LeaderboardInfo, 10>::vf12

/// Looks up the caller's entry index in this board's value table, then
/// finds that value's position in the board's key table.
/// ///
/// The loader callee (id `LEADERBOARD_ID`) fills three out words through a
/// frame pointer: the key count (at `+0x04`), the key array (at `+0x08`)
/// and the value array (at `+0x14`). When the loader reports failure the
/// result is -1. Otherwise `values[index]` is read (a -1 there also yields
/// -1, as does a zero count) and the keys are scanned with an unsigned
/// bound for the first equal word; the result is its index, or -1 when no
/// key matches.
/// ///
/// Original: 0x005607b0 (stdcall, one stack word). Episodic race board 68;
/// only the loader id differs between instantiations.
lf_checker_rt::export!(stdcall, rw_005607b0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10f;
        const OUT_COUNT: usize = 1;
        const OUT_KEYS: usize = 2;
        const OUT_VALUES: usize = 5;
        const MISSING: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return MISSING;
        }
        let values = out[OUT_VALUES] as *const u32;
        let want = values.add(index as usize).read_unaligned();
        if want == MISSING {
            return MISSING;
        }
        let count = out[OUT_COUNT];
        if count == 0 {
            return MISSING;
        }
        let keys = out[OUT_KEYS] as *const u32;
        let mut i = 0u32;
        loop {
            if keys.add(i as usize).read_unaligned() == want {
                return i;
            }
            i += 1;
            if i >= count {
                return MISSING;
            }
        }
    }

});
