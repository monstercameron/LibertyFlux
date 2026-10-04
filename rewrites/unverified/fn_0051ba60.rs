// original: 0x0051ba60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race19NoHolds, player_schema::LeaderboardInfo, 10>::vf12
/// Position of a row's key inside this leaderboard's key table.
///
/// Calls the schema lookup (callee 1) with this leaderboard's id; on
/// success info holds the key count at `+0x04`, the key table at `+0x08`
/// and the value table at `+0x14`. Takes `values[index]`; a value of -1
/// means no row and returns -1, as does an empty table. Otherwise returns
/// the first key-table index holding that value, or -1.
///
/// Arguments: `index`, the row. The incoming object pointer is unused.
/// Original: thiscall, one stack word; the count check and loop bound are
/// unsigned, matching the original's conditional jumps.
lf_checker_rt::export!(thiscall, rw_0051ba60(_this: u32, index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0x70;
        const LOOKUP_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 1;
        const KEYS_WORD: usize = 2;
        const VALUES_WORD: usize = 5;
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
        let values = info[VALUES_WORD];
        let v = rd32(values.wrapping_add(index.wrapping_mul(4)));
        if v == NONE {
            return NONE;
        }
        let count = info[COUNT_WORD];
        if count == 0 {
            return NONE;
        }
        let keys = info[KEYS_WORD];
        let mut i = 0u32;
        while i < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == v {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NONE
    }
});
