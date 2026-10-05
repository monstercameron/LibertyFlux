// original: 0x00520d70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race38NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Leaderboard reverse lookup: position of an indexed entry in the key list.
///
/// Takes the entry at `index` from leaderboard `0x7b`'s lookup list and
/// returns its position in the key list, or all-ones when the lookup
/// fails, the entry is the empty marker, the key list is empty or no key
/// matches. The key-list bound is unsigned.
///
/// Original: 0x00520D70 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00520d70(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0x7b;
        const INFO_CALLEE: u32 = 1;
        const INFO_COUNT: usize = 1;
        const INFO_KEYS: usize = 2;
        const INFO_LOOKUP: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT];
        let keys = info[INFO_KEYS];
        let lookup = info[INFO_LOOKUP];
        let v = rd32(lookup.wrapping_add(index.wrapping_mul(4)));
        if v == NOT_FOUND {
            return NOT_FOUND;
        }
        // Unsigned bound exactly as the original's loop; count 0 ends at once.
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i: u32 = 0;
        loop {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == v {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
