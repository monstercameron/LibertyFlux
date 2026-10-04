// original: 0x0051cf50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race23NoHolds, player_schema::LeaderboardInfo, 10>::vf9
/// Row-type code of one leaderboard row value.
///
/// Looks the row value up through the schema lookup (this leaderboard's
/// id; the value table is info word `+0x14`, indexed by the stack
/// argument), classifies it through a second callee, then maps the class
/// to a code: 1 -> 0, 2 -> 1, 3 -> 3, 5 -> 2, anything else (including
/// lookup failure and class -1) -> -1.
///
/// Arguments: `index`, the row. The incoming object pointer is unused.
/// Original: thiscall, one stack word; the class switch is a jump table
/// in the original, a match here.
lf_checker_rt::export!(thiscall, rw_0051cf50(_this: u32, index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0x74;
        const LOOKUP_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const TABLE_WORD: usize = 5;
        const NONE: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NONE;
        }
        let table = info[TABLE_WORD];
        let value = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, value);
        if class == NONE {
            return NONE;
        }
        match class.wrapping_sub(1) {
            0 => 0, 1 => 1, 2 => 3, 3 => NONE, 4 => 2, _ => NONE,
        }
    }
});
