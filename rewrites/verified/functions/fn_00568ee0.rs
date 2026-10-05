// original: 0x00568EE0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_99, player_schema::LeaderboardInfo, 10>::vf2

/// Virtual method of a `rlConcreteLeaderboardInfo` template instantiation
/// (one ranked-episodic-race leaderboard schema): asks the object for its
/// current value through vtable slot 1 and, when it equals `expected`,
/// publishes this instantiation's board descriptor pointer through `out`
/// and returns `out`. The pointer is a data address the loader relocates,
/// so it is derived with `relocated`, never hard-coded.
/// Returns null when the value differs or `out` is null.
/// Thiscall with two stack arguments; the callee pops them (the callee pops 8 bytes).
/// Original: 0x00568EE0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00568ee0(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        /// File address of the board descriptor this instantiation publishes.
        const BOARD_FILE_VA: u32 = 0xfd9644;
        /// Vtable byte offset of the value query (slot 1).
        const VTABLE_SLOT_QUERY: u32 = 4;
        #[inline(always)]
        unsafe fn mem32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vt = mem32(this);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(mem32(vt.wrapping_add(VTABLE_SLOT_QUERY)) as usize);
        let v = query(this);
        if v != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(BOARD_FILE_VA));
        out
    }
});
