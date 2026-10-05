// original: 0x00571ad0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_131, player_schema::LeaderboardInfo, 10>::vf2

/// Key match check for the race-131 leaderboard table.
///
/// Calls the second virtual slot of the object in `obj` and compares the
/// result with `key`. When they are equal and `out` is non-null, the table's
/// tag constant 0xfcfa6c is stored through `out` and `out` is returned;
/// otherwise 0 is returned (a null `out` also returns 0, storing nothing).
/// thiscall: object in ECX, two stack arguments, pops 8.
lf_checker_rt::export!(thiscall, rw_00571ad0(obj: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4; // second virtual slot, byte offset
        const TABLE_TAG: u32 = 0xfcfa6c;
        let vtable = (obj as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = probe(obj);
        if got != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(TABLE_TAG);
        out
    }
});
