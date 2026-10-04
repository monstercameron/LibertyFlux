// original: 0x0052e7c0 leaderboard_match_tag_store_id

/// Store this leaderboard's id when the peer tag matches.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race28Standard, player_schema::LeaderboardInfo, 10>::vf2`.
///
/// `this` points to an object whose virtual slot 1 answers a tag word.
/// When that tag equals `expect` and `out` is non-null, the leaderboard
/// descriptor address `STORE_ID` (a file VA with a relocation entry, resolved
/// through `relocated`) is written to `*out` and `out` is returned; otherwise
/// the result is 0 and nothing is stored.
///
/// Calling convention: thiscall, two stack arguments, callee pops 8. The
/// indirect callee takes `this` in ECX, pops nothing, answers in EAX.
lf_checker_rt::export!(thiscall, rw_0052e7c0(this: u32, out: u32, expect: u32) -> u32 {
    /// Leaderboard descriptor address written on a tag match, as a file VA.
    const STORE_ID: u32 = 0xfced8c;
    /// Virtual slot of the tag callee (second slot, +4).
    const TAG_SLOT: u32 = 4;
    unsafe {
        let vt = (this as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(TAG_SLOT) as *const u32).read_unaligned();
        let tag_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if tag_of(this) != expect {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(STORE_ID));
        out
    }
});
