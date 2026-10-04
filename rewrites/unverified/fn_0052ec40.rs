// original: 0x0052EC40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race29Standard, player_schema::LeaderboardInfo, 10>::vf12
/// Index of a leaderboard value in the key array (race 29, board id 0x3b).
/// 
/// Stdcall, one stack word (`index`). Asks the fetch callee (fastcall:
/// board id in ecx, out-struct in edx) for this board's tables, then
/// reads the value at `index` from the value array (+0x14) and scans
/// the key array (+0x08) for it, returning the first position.
/// Returns 0xffff_ffff when the fetch fails (al == 0), the value is
/// -1, the count (+0x04) is zero, or no key matches. The scan bound is
/// an unsigned compare, matching the original's `jb`.
lf_checker_rt::export!(stdcall, rw_0052ec40(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x3b;
        const FETCH: u32 = 5;
        const NONE: u32 = 0xffff_ffff;
        /// Out layout the fetch callee fills: entry count at +0x04, key
        /// array at +0x08, value array at +0x14. Word +0x00 is never read.
        #[repr(C)]
        struct FetchOut {
            _unused: u32,
            count: u32,
            keys: u32,
            _gap: [u32; 3],
            values: u32,
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = FetchOut { _unused: 0, count: 0, keys: 0, _gap: [0; 3], values: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xff == 0 {
            return NONE;
        }
        let want = rd32(out.values.wrapping_add(index.wrapping_mul(4)));
        if want == NONE {
            return NONE;
        }
        let count = out.count;
        if count == 0 {
            return NONE;
        }
        let keys = out.keys;
        let mut i: u32 = 0;
        loop {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NONE;
            }
        }
    }
});
