// original: 0x005569f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_32, player_schema::LeaderboardInfo, 10>::vf2
/// Install this board's vtable into a slot word after a reader check, or 0.
///
/// Reads the vtable from `this` and calls its second entry (the row-id reader,
/// checker callee 1, thiscall with `this` in ECX). When the answer differs from
/// `expect`, or `slot` is null, the result is 0 and nothing is written.
/// Otherwise this board's vtable address is stored through `slot` and `slot`
/// is returned.
///
/// Original: 0x005569F0 (thiscall: `this` in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_005569f0(this: u32, slot: u32, expect: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00fd5ba4;
        const READER_OFF: u32 = 4;
        let vt = (this as *const u32).read();
        let reader: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vt.wrapping_add(READER_OFF) as *const u32).read() as usize,
        );
        if reader(this) != expect {
            return 0;
        }
        if slot == 0 {
            return 0;
        }
        (slot as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        slot
    }
});
