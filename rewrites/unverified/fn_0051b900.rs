// original: 0x0051b900 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race18NoHolds, player_schema::LeaderboardInfo, 10>::vf8
/// Width in bytes of one leaderboard row value.
///
/// Looks the row value up exactly like the sibling value getter (schema
/// lookup with this leaderboard's id, table at info `+0x10`, indexed by
/// the stack argument), classifies it through a second callee, then maps
/// the class to a width: class 1 takes 4 bytes, classes 2 and 3 take 8,
/// class 5 takes 4, anything else (including lookup failure and class -1)
/// takes 0.
///
/// Arguments: `index`, the row. The incoming object pointer is unused.
/// Original: thiscall, one stack word; the class switch is a jump table
/// in the original, a match here.
lf_checker_rt::export!(thiscall, rw_0051b900(_this: u32, index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0x6f;
        const LOOKUP_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const TABLE_WORD: usize = 4;
        const NONE: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let table = info[TABLE_WORD];
        let value = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, value);
        if class == NONE {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4, 1 => 8, 2 => 8, 3 => 0, 4 => 4, _ => 0,
        }
    }
});
