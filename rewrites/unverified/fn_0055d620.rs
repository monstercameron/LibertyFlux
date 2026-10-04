// original: 0x0055d620 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_56, player_schema::LeaderboardInfo, 10>::vf8
/// Report the storage width of one leaderboard column entry.
///
/// Fetches this race's schema (id `RACE_ID`) through the schema callee, which
/// fills a frame slot with the column list. The argument selects an entry; its
/// kind is classified by a second callee and mapped to a width in bytes. A
/// failed fetch, an unknown kind, or an unmapped class returns 0.
///
/// Original: 0x0055d620 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0055d620(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const RACE_ID: u32 = 0x103;
        const SCHEMA_CALLEE: u32 = 0;
        const KIND_CALLEE: u32 = 1;
        const UNKNOWN: u32 = 0xffff_ffff;
        let mut frame = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, RACE_ID, frame.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return 0;
        }
        let entry = rd32(frame[5].wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, entry);
        if kind == UNKNOWN {
            return 0;
        }
        let arm = kind.wrapping_sub(1);
        if arm > 4 {
            return 0;
        }
        match arm {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
