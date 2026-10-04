// original: 0x00579550 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_159, player_schema::LeaderboardInfo, 10>::vf2

/// Stamp the caller's slot when the object tag matches.
///
/// Calls the second virtual slot of `this`, compares the tag with
/// `expect`, and when they agree and `out` is non-null writes the marker
/// 0x00FCF1DC there. Always returns 0; the write is the only effect.
///
/// Original: 0x00579550 (thiscall/2; the indirect call runs through the same
/// fabricated object on both sides of the comparison).
lf_checker_rt::export!(thiscall, rw_00579550(this: u32, out: u32, expect: u32) -> u32 {
    unsafe {        const TAG_SLOT: u32 = 4;
        const MARKER: u32 = 0xFCF1DC;

        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(TAG_SLOT) as *const u32).read_unaligned();
        let tag: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        if tag(this) != expect {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(MARKER);
        0

    }
});
