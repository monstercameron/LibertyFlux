// original: 0x0054dcf0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_26, player_schema::LeaderboardInfo, 10>::vf8

/// Classify the leaderboard table entry at `index` into a size code.
///
/// Calls the info-fetch callee (fastcall: ECX = schema id 0xb6, EDX = info
/// buffer) which reports success in AL and fills the buffer: table pointer
/// at `+0x14`. On success the entry `table[index]` is passed (ECX) to the
/// classifier callee, whose answer maps to a size: 1 -> 4, 2 -> 8, 3 -> 8,
/// 4 -> 0, 5 -> 4. Any other answer, a classifier answer of -1, or an
/// info-fetch failure yields 0. The index is used as-is with no bounds
/// check.
///
/// Original: stdcall, one stack word; incoming ECX is unused (overwritten
/// with the schema id before the first call).
lf_checker_rt::export!(stdcall, rw_0054dcf0(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xb6;
        const TABLE_OFF: u32 = 0x14;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(0, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let table = info[(TABLE_OFF / 4) as usize];
        let v = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(1, u32, v);
        if kind == 0xffff_ffff {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
