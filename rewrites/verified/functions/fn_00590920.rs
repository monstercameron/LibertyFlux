// original: 0x00590920 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_244, player_schema::LeaderboardInfo, 10>::vf2

/// Gate on a vtable predicate, then publish this leaderboard's slot id.
///
/// Calls the gate at vtable slot `+4` of `this` (ECX) with `this` still
/// in ECX; the gate takes no stack arguments and pops nothing. When its
/// answer differs from `want`, or when `out_ptr` is null, returns 0 and
/// stores nothing. Otherwise writes `SLOT_VALUE` (this instantiation's
/// leaderboard slot id) to `out_ptr` and returns `out_ptr`.
///
/// Original: thiscall, ECX = this, two stack words, callee pops 8. The
/// gate is an indirect call through a fabricated vtable under the
/// checker; both sides land on the same scripted stub. `SLOT_VALUE` is a
/// file address in read-only data (this leaderboard's descriptor) with a
/// relocation entry, so the rewrite derives it with `relocated()` like
/// the original's relocated immediate.
lf_checker_rt::export!(thiscall, rw_00590920(this: u32, out_ptr: u32, want: u32) -> u32 {
    unsafe {
        const SLOT_VALUE_FILE_VA: u32 = 0xfdcf6c;
        const VTABLE_GATE: u32 = 4;
        let slot_value = lf_checker_rt::relocated(SLOT_VALUE_FILE_VA);
        let vtable = (this as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable.wrapping_add(VTABLE_GATE)) as *const u32).read_unaligned() as usize);
        let got = gate(this);
        if got != want {
            return 0;
        }
        if out_ptr == 0 {
            return 0;
        }
        (out_ptr as *mut u32).write_unaligned(slot_value);
        out_ptr
    }
});
