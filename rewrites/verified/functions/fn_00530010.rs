// original: 0x00530010 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race33Standard, player_schema::LeaderboardInfo, 10>::vf6
/// Position of a key in this board's key array (race 33, board id 0x8f).
/// 
/// Stdcall, one stack word (`value`). Fetches the board's tables, then
/// linearly scans the key array (+0x10) for `value`, returning the
/// first matching position. Returns 0xffff_ffff when the fetch fails
/// (al == 0), the signed count (+0x0c) is not positive, or no key
/// matches. Bound and count tests are signed, matching the original's
/// `jl`/`jle`.
lf_checker_rt::export!(stdcall, rw_00530010(value: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x8f;
        const FETCH: u32 = 5;
        const NONE: u32 = 0xffff_ffff;
        /// Out layout the fetch callee fills: entry count at +0x0c, key
        /// array at +0x10.
        #[repr(C)]
        struct FetchOut {
            _head: [u32; 3],
            count: u32,
            keys: u32,
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = FetchOut { _head: [0; 3], count: 0, keys: 0 };
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, core::ptr::addr_of_mut!(out) as u32);
        if ok & 0xff == 0 {
            return NONE;
        }
        let count = out.count;
        if (count as i32) <= 0 {
            return NONE;
        }
        let keys = out.keys;
        let mut i: u32 = 0;
        loop {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == value {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= (count as i32) {
                return NONE;
            }
        }
    }
});
