// original: 0x00586E90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_208, player_schema::LeaderboardInfo, 10>::vf7

/// Ranked leaderboard direct column read: return the column value for row
/// `index` (race 208, leaderboard id 0x1a8).
///
/// The schema helper (callee 1) is asked for leaderboard 0x1a8 with a scratch
/// out-struct; it fills the column-array pointer (word 4). The value at
/// `index` is returned directly. `this` is unused.
///
/// Edge cases: helper failure (al == 0) returns -1; `index` is used
/// unchecked, so an out-of-range row reads past the array (or faults).

/// Calling convention: thiscall with one stack word (`this` in ECX, unused).

lf_checker_rt::export!(thiscall, rw_00586e90(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a8;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        /// Word index of the column-array pointer in the out-struct.
        const W_COLUMN: usize = 4;
        let mut out = [0u32; 5];
        out[W_COLUMN] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        ((out[W_COLUMN]).wrapping_add((index).wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
