// original: 0x00524da0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race52NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Reports the storage size class of one leaderboard column.
///
/// Asks the registry for id LEADERBOARD_ID's column list (array
/// at +20), classifies entry `index` through the kind callee and
/// maps kind-1 to a size class (4 or 8 bytes, else 0), returning
/// 0 when the lookup fails, the entry has no kind, or the kind is
/// out of range. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00524da0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x41;
        const LOOKUP: u32 = 1;
        const KIND_OF: u32 = 2;
        const ARRAY: usize = 5;
        const NO_KIND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let arr = buf[ARRAY] as u32;
        let elem = (arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_OF, u32, elem);
        if kind == NO_KIND {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 0x4,
            1 => 0x8,
            2 => 0x8,
            3 => 0x0,
            4 => 0x4,
            _ => 0,
        }
    }
});
