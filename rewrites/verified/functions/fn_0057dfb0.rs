// original: 0x0057DFB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_176, player_schema::LeaderboardInfo, 10>::vf2
/// Gate on the board's live value, then publish this board's key.
///
/// `obj` points to the board object: its first word is the vtable and slot 1
/// (at `+4`) is the gate, taking no arguments and returning the board's
/// current value. When the gate's value differs from `expected`, 0 is
/// returned, as it is when the values match but `out` is null. Otherwise
/// the address of this board's registration record in read-only data
/// (`LEADERBOARD_KEY`, a file address stored relocated) is written to
/// `*out` and `out` itself is returned.
///
/// Original: 0x0057DFB0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0057DFB0(obj: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_GATE_SLOT: u32 = 0x04;
        const LEADERBOARD_KEY: u32 = 0xfcfe3c;
        let vtable = (obj as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable.wrapping_add(VTABLE_GATE_SLOT)) as *const u32).read_unaligned() as usize,
        );
        let current = gate(obj);
        if current != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(LEADERBOARD_KEY));
        out
    }
});
