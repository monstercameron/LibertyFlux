// original: 0x0058C320 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_228,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2
/// Leaderboard downcast probe: OUT if this board's tag equals ID, else null.
///
/// Arguments: THIS in ECX, OUT and ID as stack words (thiscall). Loads the
/// vtable from THIS and calls slot 1 with THIS still in ECX (a virtual call
/// the checker intercepts through a planted vtable). If the returned tag
/// differs from ID, or OUT is null, returns 0. Otherwise stores the board's
/// vtable address 0x00FDD154 (a relocated image address) at OUT and returns OUT.
/// Original: 0x0058C320 (thiscall, two stack words).

lf_checker_rt::export!(thiscall, rw_0058C320(this: u32, out: u32, id: u32) -> u32 {
    unsafe {
        const BOARD_VTABLE: u32 = 0x00FDD154;
        let vt = (this as *const u32).read_unaligned();
        let slot = ((vt.wrapping_add(4)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let tag = f(this);
        if tag != id {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(BOARD_VTABLE));
        out
    }
});
