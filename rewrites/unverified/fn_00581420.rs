// original: 0x00581420 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_188, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this board's tag when the peer kind matches.
///
/// Calls the peer object's kind slot (vtable slot +4, reached through `this`)
/// and compares the result with `expected`. On a match with a non-null `out`,
/// stores the board tag and returns `out`; otherwise returns 0 without
/// storing. The tag is an opaque board constant.
///
/// Original: thiscall (`this` in ECX), two stack words; one indirect callee
/// through the object's vtable (intercepted via a planted stub).
lf_checker_rt::export!(thiscall, rw_00581420(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        // Vtable slot of the peer kind query, in bytes.
        const KIND_SLOT: u32 = 4;
        const BOARD_TAG: u32 = 0xfd94ac;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if kind_of(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(BOARD_TAG);
        out
    }
});
