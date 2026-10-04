// original: 0x005861B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_205, player_schema::LeaderboardInfo, 10>::vf8

/// Ranked leaderboard column width: return the byte width (4 or 8) of the
/// column for row `index`, or 0 when unknown (race 205, leaderboard
/// id 0x1a5).
///
/// The schema helper (callee 1) is asked for leaderboard 0x1a5 with a scratch
/// out-struct; it fills the column-array pointer (word 5). The column value
/// at `index` is passed to the type helper (callee 2); a type of -1 yields
/// 0. Otherwise the type minus one selects a width: 1 -> 4, 2 -> 8,
/// 3 -> 8, 4 -> 0, 5 -> 4, anything else -> 0. `this` is unused.
///
/// Edge cases: helper failure (al == 0) returns 0; `index` is used
/// unchecked.

/// Calling convention: thiscall with one stack word (`this` in ECX, unused).

lf_checker_rt::export!(thiscall, rw_005861b0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a5;
        const SCHEMA_CALLEE: u32 = 1;
        const TYPE_CALLEE: u32 = 2;
        /// Word index of the column-array pointer in the out-struct.
        const W_COLUMN: usize = 5;
        let mut out = [0u32; 6];
        out[W_COLUMN] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let column = out[W_COLUMN];
        let value = ((column).wrapping_add((index).wrapping_mul(4)) as *const u32).read_unaligned();
        let ty: u32 = lf_checker_rt::callee_thiscall!(TYPE_CALLEE, u32, value);
        if ty == 0xffff_ffff {
            return 0;
        }
        match ty.wrapping_sub(1) {
            0 => 4, 1 => 8, 2 => 8, 3 => 0, 4 => 4, _ => 0,
        }
    }
});
