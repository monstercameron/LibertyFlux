// original: 0x00587460 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_210, player_schema::LeaderboardInfo, 10>::vf2

/// Leaderboard info tag check: when the object's tag matches `key`, stamp
/// the tag constant into `*out` (race 210).
///
/// The object's virtual slot 1 (double indirection through `this`) yields
/// its tag. If the tag differs from `key`, or `out` is null, the result is
/// 0 and nothing is written. Otherwise 0xfcfc24 is stored to `*out` and
/// `out` is returned.
///
/// Calling convention: thiscall with two stack words (`this` in ECX).

lf_checker_rt::export!(thiscall, rw_00587460(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const TAG_CONST: u32 = 0xfcfc24;
        /// Byte offset of the tag getter in the object's vtable.
        const VTABLE_SLOT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let tag_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if tag_of(this) != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(TAG_CONST);
        out
    }
});
