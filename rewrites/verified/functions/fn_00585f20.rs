// original: 0x00585F20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_205, player_schema::LeaderboardInfo, 10>::vf13

/// Ranked leaderboard mapped lookup: map key `key` through the id array to
/// its mapped value (race 205, leaderboard id 0x1a5).
///
/// The schema helper (callee 1) is asked for leaderboard 0x1a5 with a scratch
/// out-struct; it fills the entry count (word 3), the id-array pointer
/// (word 4) and the mapped-value-array pointer (word 5). A non-positive
/// count yields -1. Otherwise `key` is located in the first `count` ids
/// with a signed linear search; on a hit the mapped value at the same
/// position is returned, on a miss -1. `this` is unused. (The original
/// re-checks the found index against -1 after the search; that branch is
/// dead, since a found index is always >= 0, and is not reproduced.)
///
/// Edge cases: helper failure (al == 0) returns -1.

/// Calling convention: thiscall with one stack word (`this` in ECX, unused).

lf_checker_rt::export!(thiscall, rw_00585f20(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a5;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        /// Word indexes in the schema helper's out-struct.
        const W_COUNT: usize = 3;
        const W_IDS: usize = 4;
        const W_MAPPED: usize = 5;
        let mut out = [0u32; 6];
        out[W_COUNT] = 0;
        out[W_IDS] = 0;
        out[W_MAPPED] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = out[W_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = out[W_IDS];
        let mapped = out[W_MAPPED];
        let mut i = 0i32;
        loop {
            if ((ids).wrapping_add(((i as u32)).wrapping_mul(4)) as *const u32).read_unaligned() == key {
                return ((mapped).wrapping_add(((i as u32)).wrapping_mul(4)) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
