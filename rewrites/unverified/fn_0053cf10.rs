// original: 0x0053CF10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamGangBase_BG, player_schema::LeaderboardInfo, 10>::vf2

/// Check the leaderboard's live row count and tag the caller's slot.
///
/// Calls the second virtual slot of the object in ECX (a filler that reports
/// the current row count with no arguments). When the report equals the
/// expected count and the caller's out pointer is non-null, writes the
/// leaderboard tag constant into it and returns the pointer; otherwise
/// returns null. A null out pointer with a matching count still returns null.
///
/// Original: thiscall with two stack words (out pointer, expected count).

lf_checker_rt::export!(thiscall, rw_0053cf10(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        let tag: u32 = lf_checker_rt::relocated(0x00FDD_B5C);
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable as *const u32).wrapping_add(1).read_unaligned();
        let filler: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = filler(this);
        if got != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(tag);
        out
    }
});
