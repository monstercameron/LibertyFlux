// original: 0x00586610 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_206, player_schema::LeaderboardInfo, 10>::vf8

/// Leaderboard column width: fetch this board's table, classify one slot.
///
/// The slot value at `index` is classified through the kind helper
/// (callee 2, tag in ECX), and the kind maps to a byte width: kinds 1 and 5
/// take 4 bytes, kinds 2 and 3 take 8, anything else takes 0, including the
/// missing-kind -1 and kind 0. A failed fetch also yields 0. `this` is
/// unused.
///
/// Fetch protocol, shared by every virtual slot of this board family: call
/// the fetch helper (callee 1) with `LEADERBOARD_ID` in ECX and a pointer to
/// a zeroed out block in EDX. Only the low byte of the answer decides
/// success. On success the block holds the table pointer at `+0x14`; the
/// other words are never read. The kind map above is the original's jump
/// table, read entry by entry.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00586610(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a6;
        const FETCH_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const WIDTH_NARROW: u32 = 4;
        const WIDTH_WIDE: u32 = 8;

        #[repr(C)]
        struct FetchOut {
            _head: [u32; 5],
            table: u32,
        }
        let mut out = FetchOut { _head: [0; 5], table: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let slot =
            (out.table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, slot);
        match kind {
            1 | 5 => WIDTH_NARROW,
            2 | 3 => WIDTH_WIDE,
            _ => 0,
        }
    }
});
