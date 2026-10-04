// original: 0x00576df0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_150, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's id through an out-pointer on match.
///
/// Calls the virtual slot at `this+4` with `this` (vtable slot 1) and
/// compares its answer with `expect`. When they differ, or when `out_ptr`
/// is null, returns 0. Otherwise stores this instantiation's id
/// (0xfd13ec) at `out_ptr` and returns `out_ptr`.
///
/// Original: 0x00576df0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00576df0(this: u32, out_ptr: u32, expect: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x04;
        const PUBLISHED_ID: u32 = 0xfd13ec;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let getter: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if getter(this) != expect {
            return 0;
        }
        if out_ptr == 0 {
            return 0;
        }
        (out_ptr as *mut u32).write_unaligned(PUBLISHED_ID);
        out_ptr
    }
});
