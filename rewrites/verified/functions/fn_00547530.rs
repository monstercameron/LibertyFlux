// original: 0x00547530 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_3, player_schema::LeaderboardInfo, 10>::vf2
/// Board-descriptor check for one ranked leaderboard board (virtual slot 2).
///
/// Calls the second entry of this object's virtual table (the base-class id
/// query) with `this` in ECX. When the answer equals `expected` and `out` is
/// non-null, stores the board descriptor pointer (file VA 0xFD7AE4, relocated
/// at runtime) at `out` and returns `out`; otherwise returns null (mismatch,
/// or null `out`, both yield null without storing).
///
/// Calling convention: thiscall with `this` in ECX and two stack words
/// (`out`, `expected`), callee cleans 8 bytes. No memory is read besides the
/// vtable pointer, the slot, and the two stack words; the only write is the
/// descriptor store on the success path.
lf_checker_rt::export!(thiscall, rw_00547530(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: usize = 1; // second entry: base-class id query
        const BOARD_DESC_FILE_VA: u32 = 0xFD7AE4;
        let vtable = (this as *const u32).read() as *const u32;
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(vtable.add(VTABLE_SLOT).read() as usize);
        if query(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write(lf_checker_rt::relocated(BOARD_DESC_FILE_VA));
        out
    }
});
