// original: 0x0051e410 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race28NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Position lookup: fetch this leaderboard's id table and return the
/// position of `want` in it.
///
/// Calls the table-fetch callee with the leaderboard index
/// (`LEADERBOARD_INDEX`) in ECX and a scratch frame in EDX; the callee
/// fills `count` at frame `+12` and the id-table pointer at `+16`.
/// Returns `MISSING` (-1) when the fetch reports failure or `count` is
/// zero or negative; otherwise the zero-based position of `want` in the
/// first `count` ids (signed bound), or `MISSING` when absent.
///
/// Original: 0x0051e410 (thiscall, one stack word; `this` is unread).
lf_checker_rt::export!(thiscall, rw_0051e410(this: u32, want: u32) -> u32 {
    unsafe {
        const LEADERBOARD_INDEX: u32 = 0x3e;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xffff_ffff;
        let _ = this;
        #[repr(C)]
        struct FetchOut {
            _pad0: u32,
            _pad1: u32,
            _pad2: u32,
            count: u32,
            ids: u32,
        }
        let mut out: FetchOut = core::mem::zeroed();
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE,
            u32,
            LEADERBOARD_INDEX,
            (&mut out as *mut FetchOut) as u32
        );
        if (ok & 0xff) == 0 {
            return MISSING;
        }
        let n = out.count as i32;
        if n <= 0 {
            return MISSING;
        }
        let mut i = 0u32;
        loop {
            let v =
                ((out.ids.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if v == want {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= n {
                return MISSING;
            }
        }
    }
});
