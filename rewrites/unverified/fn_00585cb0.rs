// original: 0x00585CB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_204, player_schema::LeaderboardInfo, 10>::vf6

/// Leaderboard id search: fetch this board's id list, return `key`'s slot.
///
/// Returns `MISSING` (-1) when the fetch fails, the count is zero or
/// negative (compared SIGNED), or the key is absent. `this` is unused.
///
/// Fetch protocol, shared by every virtual slot of this board family: call
/// the fetch helper (callee 1) with `LEADERBOARD_ID` in ECX and a pointer to
/// a zeroed out block in EDX. Only the low byte of the answer decides
/// success. On success the block holds the id count at `+0x0C` and the
/// id-list pointer at `+0x10`; the other words are never read. The scan
/// bound is compared signed.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00585CB0(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a4;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;

        #[repr(C)]
        struct FetchOut {
            _head: [u32; 3],
            count: u32,
            ids: u32,
        }
        let mut out = FetchOut { _head: [0; 3], count: 0, ids: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let count = out.count as i32;
        if count <= 0 {
            return MISSING;
        }
        let ids = out.ids;
        let mut i: u32 = 0;
        while (i as i32) < count {
            let v = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
