// original: 0x00561040 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_70, player_schema::LeaderboardInfo, 10>::vf2

/// Verifies a leaderboard key through the secondary info object, then
/// publishes this board's vtable pointer to the caller's out slot.
/// 
/// `this` points to the board info object whose first word is a pointer to
/// its secondary vtable; slot 1 (at `+0x04`) is called with `this` and
/// returns the live key. When that key equals `key` and `out` is non-null,
/// the board vtable address (`VTABLE`, relocated) is stored to `*out`
/// and `out` is returned; otherwise the result is null (a null `out` or
/// a key mismatch both yield 0, and a mismatch stores nothing).
/// 
/// Original: 0x00561040 (thiscall: this in ECX, two stack words, callee
/// pops 8). Episodic race board 70 of the template family; only the
/// vtable address differs between instantiations.
lf_checker_rt::export!(thiscall, rw_00561040(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x04;
        const VTABLE: u32 = 0x00fd023c;
        let vtable = (this as *const u32).read_unaligned();
        let live_key: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + VTABLE_SLOT) as *const u32).read_unaligned() as usize);
        if live_key(this) != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        out
    }

});
