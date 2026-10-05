// original: 0x0054C850 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_22, player_schema::LeaderboardInfo, 10>::vf2

/// Guarded board-tag store: write this board's tag when the key matches.
///
/// Calls the first virtual slot of `this` (a thiscall taking no stack
/// arguments) and compares its answer with `want`. On a mismatch, or when
/// `stamp` is null, returns 0 and writes nothing. On a match with a live
/// `stamp`, stores this board's tag there and returns `stamp`.
///
/// The tag is an address into the original's image: the immediate carries a
/// relocation entry, so the worker's relocated copy differs from the file
/// value by the load delta. It must be derived with `relocated()` from the
/// file VA; storing the file value is the classic wrong version.
///
/// Original: thiscall, ECX = this, two stack words, the callee pops 8 bytes.
lf_checker_rt::export!(thiscall, rw_0054C850(this: u32, stamp: u32, want: u32) -> u32 {    unsafe {
        const TAG_FILE_VA: u32 = 0xfd0b64;
        const VT_SLOT: u32 = 4;
        let tag = lf_checker_rt::relocated(TAG_FILE_VA);
        let vt = (this as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt.wrapping_add(VT_SLOT)) as *const u32).read_unaligned() as usize);
        let got = probe(this);
        if got != want {
            return 0;
        }
        if stamp == 0 {
            return 0;
        }
        (stamp as *mut u32).write_unaligned(tag);
        stamp
    }});
