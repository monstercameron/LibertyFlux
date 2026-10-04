// original: 0x00531190 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race37Standard, player_schema::LeaderboardInfo, 10>::vf6

/// Find a leaderboard row id in the row-id array, returning its position.
///
/// `key` is the row id to find. The helper (callee 1) is asked for the
/// row-id table of kind 0x84: it fills a scratch struct whose word at
/// `+0x0c` (INFO_COUNT) is the entry count and whose word at `+0x10`
/// (INFO_IDS) points at the array of row ids. A zero answer byte means
/// failure and yields -1. Otherwise the array is scanned from index 0
/// while the index is signed-less than the count; the first index whose
/// entry equals `key` is returned, or -1 when the count is not positive
/// or nothing matches.
///
/// Original: 0x00531190 (stdcall, one stack word; the incoming ECX, the
/// virtual-method receiver, is overwritten before any read and ignored).
lf_checker_rt::export!(stdcall, rw_00531190(key: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x84;
        const HELPER: u32 = 1;
        const INFO_COUNT: usize = 3;
        const INFO_IDS: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok = lf_checker_rt::callee_fastcall!(HELPER, u32, KIND, info.as_mut_ptr() as u32) as u8;
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT] as i32;
        let ids = info[INFO_IDS];
        if count <= 0 {
            return NOT_FOUND;
        }
        let mut i = 0i32;
        while i < count {
            let entry = rd32(ids.wrapping_add((i as u32).wrapping_mul(4)));
            if entry == key {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
