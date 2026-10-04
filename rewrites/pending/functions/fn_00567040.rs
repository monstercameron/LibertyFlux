// original: 0x00567040 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_92, player_schema::LeaderboardInfo, 10>::vf2

/// Publish the leaderboard schema pointer when the revision matches.
///
/// Arguments: the object, an output slot (may be null), and the
/// expected revision.
///
/// Calls the revision slot of the object (second virtual slot) with
/// no arguments. When its answer equals the expected revision and
/// the output slot is present, stores the schema constant 0xfdc3f4
/// there and returns the slot; otherwise returns null.
///
/// Edge cases: a revision mismatch, or a null slot with a match,
/// both return null (0).
///
/// Original: thiscall, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00567040(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const SCHEMA_FILE_VA: u32 = 0xfdc3f4;
        const REVISION_SLOT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(REVISION_SLOT) as *const u32).read_unaligned();
        let revision_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if revision_of(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(SCHEMA_FILE_VA));
        out
    }
});
