// original: 0x00531f40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race40Standard, player_schema::LeaderboardInfo, 10>::vf8

/// Look up one leaderboard entry and classify it through a five-way map.
///
/// Calls the leaderboard fetch callee (fastcall: board id in ECX, out-struct
/// pointer in EDX) with this board's id (`BOARD_ID`). The callee answers in
/// AL and fills the out-struct's table slot (word 5). On success reads
/// `entry = table[index]` (32-bit wraparound addressing, no bounds check)
/// and passes it to the rank callee (object call, entry in ECX), whose
/// answer `r` is mapped: `r - 1` of 0 gives 4, of 1 or 2 gives 8, of 3
/// gives 0, of 4 gives 4; anything else (including `r == -1`, `r == 0`, or
/// `r > 5`), or a fetch failure, gives 0.
///
/// Original: 0x00531f40 (thiscall, one stack word; the entry ECX is
/// overwritten before any read, so the object pointer is ignored; the map is
/// a jump table in the original, a match here).
lf_checker_rt::export!(thiscall, rw_00531f40(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x98;
        const FETCH_CALLEE: u32 = 1;
        const RANK_CALLEE: u32 = 2;
        const TABLE_SLOT: usize = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 8];
        out[TABLE_SLOT] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE,
            u8,
            BOARD_ID,
            out.as_mut_ptr() as u32
        );
        if ok == 0 {
            return 0;
        }
        let table = out[TABLE_SLOT];
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let r: u32 = lf_checker_rt::callee_thiscall!(RANK_CALLEE, u32, entry);
        if r == 0xffff_ffff {
            return 0;
        }
        match r.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
