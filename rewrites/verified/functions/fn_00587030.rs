// original: 0x00587030 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_209, player_schema::LeaderboardInfo, 10>::vf12

/// Ranked leaderboard column lookup: return the position, in the id array,
/// of the column value for row `index` (race 209, leaderboard id 0x1a9).
///
/// The schema helper (callee 1) is asked for leaderboard 0x1a9 with a scratch
/// out-struct; it fills the entry count (word 1), the id-array pointer
/// (word 2) and the column-array pointer (word 5). The column value at
/// `index` is read; a value of -1 (no entry for the row) or a zero count
/// yields -1 without searching. Otherwise the value is located in the first
/// `count` ids with an unsigned linear search, returning its index, or -1
/// when absent. `this` is unused.
///
/// Edge cases: helper failure (al == 0) returns -1; `index` is used
/// unchecked, so an out-of-range row reads past the array (or faults).

/// Calling convention: thiscall with one stack word (`this` in ECX, unused).

lf_checker_rt::export!(thiscall, rw_00587030(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a9;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        /// Word indexes in the schema helper's out-struct.
        const W_COUNT: usize = 1;
        const W_IDS: usize = 2;
        const W_COLUMN: usize = 5;
        let mut out = [0u32; 6];
        out[W_COUNT] = 0;
        out[W_IDS] = 0;
        out[W_COLUMN] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let column = out[W_COLUMN];
        let want = ((column).wrapping_add((index).wrapping_mul(4)) as *const u32).read_unaligned();
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = out[W_COUNT];
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = out[W_IDS];
        let mut i = 0u32;
        loop {
            if ((ids).wrapping_add((i).wrapping_mul(4)) as *const u32).read_unaligned() == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
