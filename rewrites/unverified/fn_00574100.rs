// original: 0x00574100 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_139, player_schema::LeaderboardInfo, 10>::vf8

/// Ranked-race leaderboard row size class: fetch this board's row
/// table, classify the row at `index` through the shared classifier, and
/// map its key to a size (1 to 4, 2 and 3 to 8, 5 to 4, anything else
/// to 0, via the original's jump table).
///
/// `LEADERBOARD_ID` selects the board; `this` is ignored. Returns 0 when
/// the fetch fails or the classifier reports -1.
///
/// Original: 0x00574100 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00574100(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x163;
        const DESC_TABLE: usize = 5;
        const SIZE_CLASS: [u32; 5] = [4, 8, 8, 0, 4];
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        desc[DESC_TABLE] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let table = desc[DESC_TABLE];
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let key: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if key == 0xffff_ffff {
            return 0;
        }
        let slot = key.wrapping_sub(1);
        if slot > 4 {
            return 0;
        }
        SIZE_CLASS[slot as usize]
    }
});
