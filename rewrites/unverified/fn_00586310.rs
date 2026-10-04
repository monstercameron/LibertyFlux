// original: 0x00586310 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_206, player_schema::LeaderboardInfo, 10>::vf12

/// Leaderboard row lookup: fetch this board's tables, then locate one row id.
///
/// `index` selects a row id from the board's row table; the function returns
/// that id's position in the board's id list, or `MISSING` (-1) when the
/// fetch fails, the row id itself is missing, the list is empty, or the id
/// is not listed. `this` is unused.
///
/// Fetch protocol, shared by every virtual slot of this board family: call
/// the fetch helper (callee 1) with `LEADERBOARD_ID` in ECX and a pointer to
/// a zeroed out block in EDX. Only the low byte of the answer decides
/// success. On success the block holds the id count at `+0x04`, the id-list
/// pointer at `+0x08` and the row-table pointer at `+0x14`; the other words
/// are never read. The search bound is compared unsigned, and `count == 0`
/// exits first, so the loop index cannot wrap.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00586310(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a6;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;

        #[repr(C)]
        struct FetchOut {
            _head: u32,
            count: u32,
            ids: u32,
            _mid: [u32; 2],
            rows: u32,
        }
        let mut out = FetchOut { _head: 0, count: 0, ids: 0, _mid: [0; 2], rows: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let needle =
            (out.rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if needle == MISSING {
            return MISSING;
        }
        let count = out.count;
        if count == 0 {
            return MISSING;
        }
        let ids = out.ids;
        let mut i: u32 = 0;
        while i < count {
            let v = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == needle {
                return i;
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
