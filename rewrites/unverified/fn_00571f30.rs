// original: 0x00571f30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_132, player_schema::LeaderboardInfo, 10>::vf2

/// Key match check for the race-132 leaderboard table.
///
/// Calls the second virtual slot of the object in `obj` and compares the
/// result with `key`. When they are equal and `out` is non-null, the table's
/// tag constant 0xfcf51c is stored through `out` and `out` is returned;
/// otherwise 0 is returned (a null `out` also returns 0, storing nothing).
/// The tag is image-relative (its immediate carries a relocation), so the
/// stored value is the relocated address, derived with `relocated`.
/// thiscall: object in ECX, two stack arguments, pops 8.
lf_checker_rt::export!(thiscall, rw_00571f30(obj: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4; // second virtual slot, byte offset
        // The tag is an image-relative constant: the original's immediate has
        // a relocation entry, so it stores the relocated address, not the raw
        // file value. Derive it the same way.
        let table_tag: u32 = lf_checker_rt::relocated(0xfcf51c);
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
        (out as *mut u32).write_unaligned(table_tag);
        out
    }
});
