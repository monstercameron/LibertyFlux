// original: 0x005456a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_14, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this board's schema descriptor when ID matches.
/// 
/// Calls the id query in the object's second vtable slot (thiscall, object
/// in ECX) and compares the answer with ID. Returns null unless they match
/// and OUT is non-null; otherwise stores the board's schema descriptor
/// address at OUT and returns OUT.
///
/// Calling convention: thiscall (object in ECX) with two stack arguments.
///
/// Original: 0x005456a0 (thiscall, object in ECX plus two stack words).
lf_checker_rt::export!(thiscall, rw_005456a0(this: u32, out: u32, id: u32) -> u32 {
    unsafe {
        /// Board schema descriptor stored at OUT. This is the raw immediate
        /// from the original's store instruction; it has no reloc entry, so
        /// the original stores the value as-is.
        const SCHEMA_DESC: u32 = 0x00fcf37c;
        /// Vtable slot of the id query: the second entry.
        const ID_QUERY_SLOT: u32 = 4;

        let vtable = (this as *const u32).read_unaligned();
        let target =
            (vtable.wrapping_add(ID_QUERY_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if query(this) != id {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(SCHEMA_DESC);
        out
    }
});
