// original: 0x0052FC50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race32Standard, player_schema::LeaderboardInfo, 10>::vf8
/// Size class of the entry at `index` (race 32, board id 0x81).
/// 
/// Stdcall, one stack word (`index`). Fetches the board's table
/// (+0x14), classifies the word at `index` through the classify
/// callee (thiscall, value in ecx), and maps the 1-based answer onto
/// {4, 8, 8, 0, 4} for answers 1..5, read from the original's jump
/// table. Returns 0 when the fetch fails, the answer is -1, or the
/// answer minus one is above 4 (unsigned).
lf_checker_rt::export!(stdcall, rw_0052fc50(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x81;
        const FETCH: u32 = 5;
        const CLASSIFY: u32 = 6;
        /// Out layout the fetch callee fills: word array at +0x14.
        #[repr(C)]
        struct FetchOut {
            _head: [u32; 5],
            table: u32,
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = FetchOut { _head: [0; 5], table: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let val = rd32(out.table.wrapping_add(index.wrapping_mul(4)));
        let r: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, val);
        if r == 0xffff_ffff {
            return 0;
        }
        let k = r.wrapping_sub(1);
        if k > 4 {
            return 0;
        }
        match k {
            0 | 4 => 4, 1 | 2 => 8, _ => 0,
        }
    }
});
