// original: 0x00585E80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_205, player_schema::LeaderboardInfo, 10>::vf2

/// Interface query: publish this interface's table when the id matches.
///
/// Calls the object's own id getter (slot 1 of its table, `this` in ECX).
/// When the answer equals `want_id` and `out` is non-null, the interface's
/// table address is stored through `out` and `out` is returned; otherwise
/// NULL is returned and nothing is stored.
///
/// The published table address is a link-time constant that the original
/// stores relocated (its HIGHLOW entry is applied like any other), so the
/// rewrite derives it with `relocated()` rather than a literal. The indirect
/// call runs through the same fabricated table on both sides and lands on
/// the same stub.
///
/// Original: thiscall, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00585E80(this: u32, out: u32, want_id: u32) -> u32 {
    unsafe {
        const ID_SLOT_BYTES: u32 = 4;
        const IFACE_TABLE_FILE_VA: u32 = 0x00FDCE9C;

        let table = (this as *const u32).read_unaligned();
        let get_id: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (table.wrapping_add(ID_SLOT_BYTES) as *const u32).read_unaligned() as usize);
        let got: u32 = get_id(this);
        if got != want_id {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(IFACE_TABLE_FILE_VA));
        out
    }
});
