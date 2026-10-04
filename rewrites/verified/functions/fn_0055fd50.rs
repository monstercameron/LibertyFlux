// original: 0x0055fd50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_65, player_schema::LeaderboardInfo, 10>::vf7

/// Reads one entry from this board's value table by index.
/// 
/// The loader callee (id `LEADERBOARD_ID`) fills one out word through a
/// frame pointer: the value array base (at `+0x10`). When the loader
/// reports failure the result is -1, otherwise it is `base[index]`.
/// 
/// Original: 0x0055fd50 (stdcall, one stack word). Episodic race board 65;
/// only the loader id differs between instantiations.
lf_checker_rt::export!(stdcall, rw_0055fd50(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10c;
        const OUT_VALUES: usize = 4;
        const MISSING: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return MISSING;
        }
        let values = out[OUT_VALUES] as *const u32;
        values.add(index as usize).read_unaligned()
    }

});
