// original: 0x0052F7B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race31Standard, player_schema::LeaderboardInfo, 10>::vf7
/// Word at `index` in this board's table (race 31, board id 0x80).
/// 
/// Stdcall, one stack word (`index`). Fetches the board's table
/// address (+0x10) and returns the word at `index`, with no bounds
/// check: the address wraps mod 2^32 exactly as the original's
/// `[base + index*4]` addressing does. Returns 0xffff_ffff only when
/// the fetch fails (al == 0).
lf_checker_rt::export!(stdcall, rw_0052f7b0(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x80;
        const FETCH: u32 = 5;
        const NONE: u32 = 0xffff_ffff;
        /// Out layout the fetch callee fills: word array at +0x10.
        #[repr(C)]
        struct FetchOut {
            _head: [u32; 4],
            table: u32,
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = FetchOut { _head: [0; 4], table: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xff == 0 {
            return NONE;
        }
        rd32(out.table.wrapping_add(index.wrapping_mul(4)))
    }
});
