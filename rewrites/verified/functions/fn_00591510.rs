// original: 0x00591510 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_246, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard row key into a width of 0, 4 or 8.
///
/// Asks the row-table helper (fastcall callee 1: ECX = `LEADERBOARD_ID`,
/// EDX = out-pointer) for this leaderboard's array; the helper answers
/// nonzero on success and fills the array pointer at out `+0x14`. A
/// failed fetch returns 0.
///
/// On success reads `key = array[arg]` (unchecked, 32-bit wrapping
/// address) and passes it in ECX to the classifier (callee 2, no stack
/// arguments). Answer -1 returns 0; otherwise the answer minus one must
/// be in 0..=4 (unsigned), else 0. The in-range answers map through a
/// five-entry jump table: 1 -> 4, 2 -> 8, 3 -> 8, 4 -> 0, 5 -> 4. The
/// table's displacement and entries all carry relocations, so the
/// dispatch works at any image base.
///
/// Original: stdcall, one stack word, callee pops 4. Incoming ECX is dead.
lf_checker_rt::export!(stdcall, rw_00591510(arg: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1ce;
        const OUT_ARR: usize = 5;
        const WIDTH4: u32 = 4;
        const WIDTH8: u32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut query = [0u32; 8];
        query[OUT_ARR] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let arr = query[OUT_ARR];
        let key = rd32(arr.wrapping_add(arg.wrapping_mul(4)));
        let v: u32 = lf_checker_rt::callee_thiscall!(2, u32, key);
        if v == 0xFFFF_FFFF {
            return 0;
        }
        match v.wrapping_sub(1) {
            0 => WIDTH4,
            1 => WIDTH8,
            2 => WIDTH8,
            3 => 0,
            4 => WIDTH4,
            _ => 0,
        }
    }
});
