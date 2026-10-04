// original: 0x56a060 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_103, player_schema::LeaderboardInfo, 10>::vf2
/// Interface match for one ranked leaderboard. (`vf2 Race103`).
///
/// Calls the object's own id query through its vtable slot 1 and compares the answer
/// with `want`. On a match with a non-null `out`, installs this class's vtable
/// pointer there and returns `out`; otherwise returns 0.
/// Vtable file VA: 0xfdf89c (relocated at load).
export!(thiscall, rw_0056a060(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        let vtbl = (this as *const u32).read();
        let slot = (vtbl.wrapping_add(4) as *const u32).read();
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if get_id(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write(relocated(0xfdf89c));
        out
    }
});
