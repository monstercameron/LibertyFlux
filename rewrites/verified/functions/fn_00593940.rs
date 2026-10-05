// original: 0x00593940 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_255, player_schema::LeaderboardInfo, 10>::vf2

/// Confirm the leaderboard id through the object's slot-1 hook, stamp the out slot.
///
/// `this` points to an object whose vtable slot 1 returns this leaderboard's
/// id. When it equals `want` and `out` is non-null, writes `MARK` to `out` and
/// returns `out`; otherwise returns null (id mismatch and null `out` share the
/// null path). `MARK` is an image address, so the original's immediate carries
/// a relocation and the rewrite derives it with `relocated` like any pointer.
/// Original: thiscall, `this` in ecx plus two stack words, the callee pops 8 bytes.
lf_checker_rt::export!(thiscall, rw_00593940(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const MARK_VA: u32 = 0xfdd2a4;
        let mark: u32 = lf_checker_rt::relocated(MARK_VA);
        let vtable = (this as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vtable.wrapping_add(4) as *const u32).read_unaligned() as usize);
        let got = hook(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(mark);
        out
    }
});
