// original: 0x00585F20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_205, player_schema::LeaderboardInfo, 10>::vf13

/// Leaderboard value lookup: fetch this board's tables, find `key` in the id
/// list, return the value stored at the same slot of the value table.
///
/// Returns `MISSING` (-1) when the fetch fails, the count is zero or
/// negative (compared SIGNED), or the key is absent. `this` is unused.
///
/// Fetch protocol, shared by every virtual slot of this board family: call
/// the fetch helper (callee 1) with `LEADERBOARD_ID` in ECX and a pointer to
/// a zeroed out block in EDX. Only the low byte of the answer decides
/// success. On success the block holds the id count at `+0x0C`, the id-list
/// pointer at `+0x10` and the value-table pointer at `+0x14`; the other
/// words are never read. The scan bound is compared signed. The original
/// re-tests the found slot against -1 after a hit; that test can never fire
/// (the slot is always below the positive count) and is omitted.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00585F20(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a5;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;

        #[repr(C)]
        struct FetchOut {
            _head: [u32; 3],
            count: u32,
            ids: u32,
            values: u32,
        }
        let mut out = FetchOut { _head: [0; 3], count: 0, ids: 0, values: 0 };
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
        let values = out.values;
        let mut i: u32 = 0;
        while (i as i32) < count {
            let v = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == key {
                return (values.wrapping_add(i.wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
