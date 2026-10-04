// original: 0x00570090 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_125, player_schema::LeaderboardInfo, 10>::vf2
use lf_checker_rt::export;
/// Publish this leaderboard's slot id when the token matches.
///
/// Asks the object for its current token through the second vtable slot;
///
/// when it equals `want`, writes this leaderboard's slot constant to
///
/// `out` and returns `out`. A mismatch or a null `out` yields 0.

export!(thiscall, rw_570090(obj: u32, out: u32, want: u32) -> u32 {
    const SLOT_VALUE: u32 = 0xfda764;
    let got = unsafe {
        let vtable = (obj as *const u32).read();
        let target = ((vtable.wrapping_add(4)) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(obj)
    };
    if got != want {
        return 0;
    }
    if out == 0 {
        return 0;
    }
    unsafe {
        (out as *mut u32).write(SLOT_VALUE);
    }
    out
});
