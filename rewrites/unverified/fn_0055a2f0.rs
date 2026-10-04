// original: 0x0055a2f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_45, player_schema::LeaderboardInfo, 10>::vf2

/// Install this board's vtable into a slot object after a reader check, or 0.
///
/// Reads the vtable pointer from `this`, calls its second entry (the row-id
/// reader, checker callee 1, thiscall, `this` still in ECX) and compares the
/// answer with `expect`. When they differ, or when `slot` is null, the result
/// is 0 and nothing is written. Otherwise this board's vtable address is
/// stored through `slot` and `slot` is returned.
///
/// `this` points to the leaderboard object, `slot` to the word receiving the
/// vtable, `expect` is the reader answer required for the install.
///
/// Original: 0x0055A2F0 (thiscall: `this` in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_0055a2f0(this: u32, slot: u32, expect: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xfda44c;
        const READER_SLOT: u32 = 4;
        let vt = (this as *const u32).read();
        let reader: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vt.wrapping_add(READER_SLOT) as *const u32).read() as usize,
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
