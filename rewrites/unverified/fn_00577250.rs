// original: 0x00577250 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_151, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's id through an out-pointer on match.
///
/// Calls the virtual slot at `this+4` with `this` (vtable slot 1) and
/// compares its answer with `expect`. When they differ, or when `out_ptr`
/// is null, returns 0. Otherwise stores this instantiation's id, a
/// relocated image address (file VA 0xfcf75c), at `out_ptr` and
/// returns `out_ptr`.
///
/// Original: 0x00577250 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00577250(this: u32, out_ptr: u32, expect: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x04;
        const PUBLISHED_ID_FILEVA: u32 = 0xfcf75c;
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
        (out_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(PUBLISHED_ID_FILEVA));
        out_ptr
    }
});
