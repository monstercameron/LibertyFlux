// original: 0x0055d230 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_55, player_schema::LeaderboardInfo, 10>::vf9
/// Report the value class of one leaderboard column entry.
///
/// Fetches this race's schema (id `RACE_ID`) through the schema callee, which
/// fills a frame slot with the column list. The argument selects an entry; its
/// kind is classified by a second callee and mapped to a class number. A
/// failed fetch, an unknown kind, or an unmapped class returns -1.
///
/// Original: 0x0055d230 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0055d230(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const RACE_ID: u32 = 0x102;
        const SCHEMA_CALLEE: u32 = 0;
        const KIND_CALLEE: u32 = 1;
        const UNKNOWN: u32 = 0xffff_ffff;
        let mut frame = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, RACE_ID, frame.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return 0xffff_ffff;
        }
        let entry = rd32(frame[5].wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, entry);
        if kind == UNKNOWN {
            return 0xffff_ffff;
        }
        let arm = kind.wrapping_sub(1);
        if arm > 4 {
            return 0xffff_ffff;
        }
        match arm {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 0xffff_ffff,
            4 => 2,
            _ => 0xffff_ffff,
        }
    }
});
