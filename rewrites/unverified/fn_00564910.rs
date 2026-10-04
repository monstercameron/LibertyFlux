// original: 0x00564910 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_83, player_schema::LeaderboardInfo, 10>::vf2
/// Identify this leaderboard object: when its tag matches `key`, store the
/// instantiation's type id into `slot` and return `slot`, else return 0.
///
/// Loads the object's virtual table from `this` and calls slot 1 (the tag
/// getter) with `this`. If the tag differs from `key`, or `slot` is null,
/// the result is 0 and nothing is stored. Otherwise `TYPE_ID` is written to
/// `slot` and `slot` itself is returned (non-null on this path).
///
/// Original: thiscall (`this` in ECX) with two stack words (slot, key).
lf_checker_rt::export!(thiscall, rw_00564910(this: u32, slot: u32, key: u32) -> u32 {
    unsafe {
        /// Type id stored by this instantiation.
        const TYPE_ID: u32 = 0xfdfef4;
        const VTABLE_TAG_SLOT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let tag_at = (vtable.wrapping_add(VTABLE_TAG_SLOT) as *const u32).read_unaligned();
        let tag_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tag_at as usize);
        if tag_of(this) != key {
            return 0;
        }
        if slot == 0 {
            return 0;
        }
        (slot as *mut u32).write_unaligned(TYPE_ID);
        slot
    }
});
