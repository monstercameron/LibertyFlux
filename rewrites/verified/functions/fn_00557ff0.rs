// original: 0x0x00557FF0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_37, player_schema::LeaderboardInfo, 10>::vf2

/// Ranked episodic race 37 leaderboard slot probe: ask the object for
/// its board id through its second virtual slot, and when the answer equals
/// the caller's expectation, tag the caller's out-word with this board's
/// class tag.
///
/// `this` points to an object whose first word is its virtual table; the
/// slot at table offset 4 is called with `this` and answers the board id.
/// When the answer differs from `want`, or `out` is null, the result is 0
/// and nothing is stored. Otherwise `CLASS_TAG` is written to `*out` and the
/// result is `out` itself. The tag is an image pointer stored as an
/// immediate with a relocation entry, so the rewrite relocates the file VA
/// instead of storing it raw.
///
/// Original: thiscall, two stack words (`out`, `want`), callee pops 8. The
/// virtual call is intercepted by a fabricated table in the proof.
lf_checker_rt::export!(thiscall, rw_00557ff0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const ID_SLOT: u32 = 4;
        const CLASS_TAG_FILE_VA: u32 = 0xfdbfbc;
        let class_tag = lf_checker_rt::relocated(CLASS_TAG_FILE_VA);
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(ID_SLOT) as *const u32).read_unaligned();
        let get_id: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let got = get_id(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(class_tag);
        out
    }
});
