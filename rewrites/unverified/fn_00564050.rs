// original: 0x00564050 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_81, player_schema::LeaderboardInfo, 10>::vf2
/// Identify this leaderboard object: when its tag matches `key`, store the
/// instantiation's type descriptor pointer into `slot` and return `slot`,
/// else return 0.
///
/// Loads the object's virtual table from `this` and calls slot 1 (the tag
/// getter) with `this`. If the tag differs from `key`, or `slot` is null,
/// the result is 0 and nothing is stored. Otherwise the type descriptor
/// pointer (a relocated image address, derived with `relocated`, never
/// hard-coded) is written to `slot` and `slot` itself is returned
/// (non-null on this path).
///
/// Original: thiscall (`this` in ECX) with two stack words (slot, key).
lf_checker_rt::export!(thiscall, rw_00564050(this: u32, slot: u32, key: u32) -> u32 {
    unsafe {
        /// File VA of this instantiation's type descriptor (.rdata).
        const TYPE_DESC_FILE_VA: u32 = 0xfcfcc4;
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
        (slot as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_DESC_FILE_VA));
        slot
    }
});
