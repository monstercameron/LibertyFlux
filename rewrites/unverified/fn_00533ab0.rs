// original: 0x00533ab0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race47Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Tag an output slot when this board's key matches the wanted key.
///
/// Calls virtual slot 1 of the object in `this` (no arguments, key in EAX)
/// and compares the key with `want`. When they are equal and `out` is not
/// null, writes the board tag `TAG` to `*out` and returns `out`. Otherwise
/// returns 0 and writes nothing.
///
/// Original: thiscall with two stack words, callee pops 8.

lf_checker_rt::export!(thiscall, rw_00533ab0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00fdb644;

        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(4) as *const u32).read_unaligned();
        let key_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if key_of(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(TAG);
        out
    }
});
