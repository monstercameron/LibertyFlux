// original: 0x00598800 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_3, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's row vtable when the caller names our board.
///
/// `this` (ECX, thiscall) is the leaderboard-info object. Its virtual slot 1
/// (dword at vtable + 4) is called with `this` and answers the board's id.
/// When that id equals `expected` and `out` is non-null, the row vtable
/// address `ROW_VTABLE` is stored through `out` and `out` itself is returned.
/// Zero is returned when the ids differ or `out` is null.
///
/// Original: 0x00598800 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00598800(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT_ID: u32 = 0x04;
        const ROW_VTABLE: u32 = 0xfd1314;

        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(VTABLE_SLOT_ID) as *const u32).read_unaligned();
        let board_id_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = board_id_of(this);
        if got != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(ROW_VTABLE));
        out
    }
});
