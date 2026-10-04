// original: 0x0051f230 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race31NoHolds, player_schema::LeaderboardInfo, 10>::vf9

/// Rank-class fetch: fetch this leaderboard's key table, classify the key
/// at `index`, and return its class number.
///
/// Calls the table-fetch callee with the leaderboard index
/// (`LEADERBOARD_INDEX`) in ECX and a scratch frame in EDX; the callee
/// fills the key-table pointer at frame `+20`. The key is classified by
/// the rank callee (id 2, key in ECX); the answer minus one indexes a
/// five-way jump table with results [0, 1, 3, 0xffff_ffff, 2].
/// Returns `MISSING` (-1) when the fetch reports failure, when the rank
/// is -1, or when the answer falls outside 1..=5.
///
/// Original: 0x0051f230 (thiscall, one stack word; `this` is unread).
lf_checker_rt::export!(thiscall, rw_0051f230(this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_INDEX: u32 = 0xa;
        const FETCH_CALLEE: u32 = 1;
        const RANK_CALLEE: u32 = 2;
        const MISSING: u32 = 0xffff_ffff;
        let _ = this;
        #[repr(C)]
        struct FetchOut {
            _pad0: u32,
            _pad1: u32,
            _pad2: u32,
            _pad3: u32,
            _pad4: u32,
            keys: u32,
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
        let key =
            ((out.keys.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        let rank: u32 = lf_checker_rt::callee_thiscall!(RANK_CALLEE, u32, key);
        if rank == MISSING {
            return MISSING;
        }
        match rank.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 0xffff_ffff,
            4 => 2,
            _ => MISSING,
        }
    }
});
