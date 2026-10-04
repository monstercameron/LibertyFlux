// original: 0x0055f930 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_64, player_schema::LeaderboardInfo, 10>::vf8

/// Classifies one entry of this board's value table into a width.
/// ///
/// The loader callee (id `LEADERBOARD_ID`) fills one out word through a
/// frame pointer: the value array base (at `+0x14`). When the loader
/// reports failure the result is 0. Otherwise `base[index]` is passed (in
/// ECX) to the rank callee; a rank of -1 yields 0, and the rank minus one
/// selects the width from [4, 8, 8, 0, 4], with any rank outside 1..=5
/// yielding 0.
/// ///
/// Original: 0x0055f930 (stdcall, one stack word; the rank switch is a
/// five-entry jump table in the original). Episodic race board 64; only
/// the loader id differs between instantiations.
lf_checker_rt::export!(stdcall, rw_0055f930(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10b;
        const OUT_VALUES: usize = 5;
        const WIDTHS: [u32; 5] = [4, 8, 8, 0, 4];
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let values = out[OUT_VALUES] as *const u32;
        let entry = values.add(index as usize).read_unaligned();
        let rank: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if rank == 0xffff_ffff {
            return 0;
        }
        let sel = rank.wrapping_sub(1);
        if sel > 4 {
            return 0;
        }
        WIDTHS[sel as usize]
    }

});
