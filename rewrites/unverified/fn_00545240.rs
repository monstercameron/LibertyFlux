// original: 0x00545240 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_13, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this board's schema descriptor when ID matches.
/// 
/// Calls the id query in the object's second vtable slot (thiscall, object
/// in ECX) and compares the answer with ID. Returns null unless they match
/// and OUT is non-null; otherwise stores the board's schema descriptor
/// address at OUT and returns OUT.
///
/// Calling convention: thiscall (object in ECX) with two stack arguments.
///
/// Original: 0x00545240 (thiscall, object in ECX plus two stack words).
lf_checker_rt::export!(thiscall, rw_00545240(this: u32, out: u32, id: u32) -> u32 {
    unsafe {
        /// File VA of the board's schema descriptor stored at OUT. The
        /// original's store immediate carries a reloc entry (proven by the
        /// checker: the worker maps the image away from its preferred base
        /// and the original stores the base-adjusted value), so derive it
        /// with relocated().
        const SCHEMA_DESC_FILE_VA: u32 = 0x00fd0d54;
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
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(SCHEMA_DESC_FILE_VA));
        out
    }
});
