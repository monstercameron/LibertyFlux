// original: 0x005865D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_206, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard row read: fetch this board's row table, return slot `index`.
///
/// There is no bounds check: the slot is read as-is. `MISSING` (-1) is
/// returned only when the fetch fails. `this` is unused.
///
/// Fetch protocol, shared by every virtual slot of this board family: call
/// the fetch helper (callee 1) with `LEADERBOARD_ID` in ECX and a pointer to
/// a zeroed out block in EDX. Only the low byte of the answer decides
/// success. On success the block holds the row-table pointer at `+0x10`;
/// the other words are never read.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_005865D0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a6;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;

        #[repr(C)]
        struct FetchOut {
            _head: [u32; 4],
            rows: u32,
        }
        let mut out = FetchOut { _head: [0; 4], rows: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        (out.rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
