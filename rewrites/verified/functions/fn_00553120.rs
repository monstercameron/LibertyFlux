// original: 0x00553120 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_19, player_schema::LeaderboardInfo, 10>::vf2

/// Stamp SLOT with this race's leaderboard address when the probe agrees.
///
/// Calls the second virtual slot of OBJ and compares the answer with WANT.
/// When they match and SLOT is non-null, writes this race's game address
/// (file value 0x00FDCDD4, relocated at load) to SLOT and returns SLOT itself;
/// otherwise returns 0 with no write. Thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_00553120(obj: u32, slot: u32, want: u32) -> u32 {
    unsafe {
        const MARK_FILE: u32 = 0x00FDCDD4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vt = rd32(obj);
        let probe: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt.wrapping_add(4)) as usize) };
        let got = probe(obj);
        if got != want {
            return 0;
        }
        if slot == 0 {
            return 0;
        }
        let mark = lf_checker_rt::relocated(MARK_FILE);
        unsafe { (slot as *mut u32).write_unaligned(mark) };
        slot
    }
});
