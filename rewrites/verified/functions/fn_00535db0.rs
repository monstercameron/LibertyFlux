// original: 0x00535db0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race55Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Claim the id slot when the probed id matches `want`.
///
/// Calls the second slot of the object's own function table (thiscall, no
/// stack words) to probe the current id. When it equals `want` and `out`
/// is non-null, writes the address of this race's descriptor record
/// (file address 0xfde134 in read-only data, relocated to the mapped image)
/// to `out` and returns `out`; otherwise returns 0 without writing.
/// The original's stored immediate carries a relocation entry, so the
/// stored value moves with the image base. Original: thiscall, two stack
/// words; callee-cleaned.
lf_checker_rt::export!(thiscall, rw_00535db0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const RECORD: u32 = 0xfde134;
        let store = lf_checker_rt::relocated(RECORD);
        let vtbl = (this as *const u32).read_unaligned();
        let slot_fn = (vtbl.wrapping_add(4) as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot_fn as usize);
        if probe(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(store);
        out
    }});
