// original: 0x00559a30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_43, player_schema::LeaderboardInfo, 10>::vf2
/// Interface match for one ranked leaderboard (`vf2 Race43`).
///
/// Calls the object's own id query through its vtable and compares the answer
/// with `want`. On a match with a non-null `out`, installs this class's
/// vtable pointer there and returns `out`; otherwise returns 0.
/// Vtable file VA: 0xfdcc7c (relocated at load).
export!(thiscall, rw_00559a30(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        let vtbl = (this as *const u32).read();
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vtbl.wrapping_add(4) as *const u32).read() as usize);
        if get_id(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write(relocated(0xfdcc7c));
        out
    }
});
