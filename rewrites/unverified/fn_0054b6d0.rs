// original: 0x0054B6D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_18, player_schema::LeaderboardInfo, 10>::vf2
/// Leaderboard-info query for one ranked leaderboard board (vtable slot 2).
///
/// Reads the object's vtable, calls its slot 1 through the object (the
/// embedded info's own query, taking `this` in ECX), and compares the answer
/// with `want`. When they match and `outptr` is non-null, stores this board's
/// subclass vtable pointer (0xfd1614) through `outptr` and returns `outptr`;
/// otherwise returns 0: on an id mismatch, or on a match with a null
/// `outptr`.
///
/// The stored vtable pointer is a relocated image address: the original
/// carries it as a relocated immediate, so the rewrite derives it with
/// `relocated` from the same file VA.
///
/// Original: thiscall with `this` in ECX and two stack words.
lf_checker_rt::export!(thiscall, rw_0054b6d0(this: u32, outptr: u32, want: u32) -> u32 {
    unsafe {
        /// File VA of this board's subclass vtable, stored through `outptr` on match.
        const SUBCLASS_VTABLE: u32 = 0xfd1614;
        /// Vtable slot of the embedded info query.
        const QUERY_SLOT: u32 = 4;
        let vtable = (this as *const u32).read();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vtable.wrapping_add(QUERY_SLOT) as *const u32).read() as usize);
        if query(this) != want {
            return 0;
        }
        if outptr == 0 {
            return 0;
        }
        (outptr as *mut u32).write(lf_checker_rt::relocated(SUBCLASS_VTABLE));
        outptr
    }
});
