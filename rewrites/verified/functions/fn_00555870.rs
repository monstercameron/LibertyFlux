// original: 0x00555870 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_28, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's stored value when the caller names it.
///
/// `this` is the leaderboard object, `out` the caller's result slot and
/// `expected` the id the caller is looking for. The object's slot-1 virtual
/// (`SLOT_GET_ID`) is called with `this` and its answer is compared against
/// `expected`; on a mismatch, or when `out` is null, the result is 0 and
/// nothing is stored. Otherwise `STORED_VALUE` (a loader-relocated file
/// constant, read through `relocated`) is written to `*out` and `out` itself
/// is returned.
///
/// Original: 0x00555870 (thiscall: this in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00555870(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const SLOT_GET_ID: u32 = 4;
        const STORED_VALUE: u32 = 0x00fdfa3c;
        let vtab = (this as *const u32).read_unaligned();
        let slot = (vtab.wrapping_add(SLOT_GET_ID) as *const u32).read_unaligned();
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if get_id(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(STORED_VALUE));
        out
    }
});
