// original: 0x005885E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_214, player_schema::LeaderboardInfo, 10>::vf2

/// Verify a leaderboard query against this board and publish its info block.
///
/// `this` points at an object whose first word is a table pointer; slot 1 of
/// that table is called with `this` and returns the board's query value. When
/// that value equals `want` and `out_ptr` is non-null, the file VA of this
/// board's info block (0x00fd8944) is stored at `out_ptr` and `out_ptr` is
/// returned. Any mismatch, or a null `out_ptr`, returns 0 and stores nothing.
///
/// Original: thiscall, two stack words, indirect call through the object's
/// table slot 1, one heap store on the match path.
lf_checker_rt::export!(thiscall, rw_005885E0(this: u32, out_ptr: u32, want: u32) -> u32 {
    unsafe {
        /// Table slot of the query call behind `this`.
        const QUERY_SLOT: u32 = 4;
        /// File VA of this board's info block, published on a match.
        const BOARD_INFO: u32 = 0x00fd8944;

        let table = (this as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((table.wrapping_add(QUERY_SLOT)) as *const u32).read_unaligned() as usize,
        );
        if query(this) != want {
            return 0;
        }
        if out_ptr == 0 {
            return 0;
        }
        (out_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(BOARD_INFO));
        out_ptr
    }
});
