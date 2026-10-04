// original: 0x0051e8d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race29NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Key fetch: fetch this leaderboard's key table and return the key at
/// `index`.
///
/// Calls the table-fetch callee with the leaderboard index
/// (`LEADERBOARD_INDEX`) in ECX and a scratch frame in EDX; the callee
/// fills the key-table pointer at frame `+16`. Returns `MISSING` (-1)
/// when the fetch reports failure, otherwise the table word at `index`
/// (no bounds check: a wild index faults, like the original).
///
/// Original: 0x0051e8d0 (thiscall, one stack word; `this` is unread).
lf_checker_rt::export!(thiscall, rw_0051e8d0(this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_INDEX: u32 = 0x45;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xffff_ffff;
        let _ = this;
        #[repr(C)]
        struct FetchOut {
            _pad0: u32,
            _pad1: u32,
            _pad2: u32,
            _pad3: u32,
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
        ((out.keys.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});
