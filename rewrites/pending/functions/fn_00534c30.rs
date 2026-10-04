// original: 0x00534C30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race51Standard, player_schema::LeaderboardInfo, 10>::vf2
/// Leaderboard interface query: ask the object for its row id through its own
/// table; when it matches `want` and `out` is given, install this board's
/// interface table and return `out`, else return null.
lf_checker_rt::export!(thiscall, rw_00534C30(this: u32, out: u32, want: u32) -> u32 {
    /// Interface table installed on a successful query (file VA).
    const IFACE_TABLE: u32 = 0x00FD2164;
    let vtable = unsafe { (this as *const u32).read() };
    let get_id: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute((vtable.wrapping_add(4) as *const u32).read() as usize) };
    if get_id(this) != want {
        return 0;
    }
    if out == 0 {
        return 0;
    }
    unsafe { (out as *mut u32).write(lf_checker_rt::relocated(IFACE_TABLE)) };
    out
});
