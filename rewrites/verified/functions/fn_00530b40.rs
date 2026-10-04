// original: 0x00530B40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race36Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Look up a leaderboard row id and return its paired value.
///
/// `key` is the row id to find. The helper (callee 1) is asked for
/// the tables of kind 0x88: it fills a scratch struct whose word at
/// `+0x0c` (INFO_COUNT) is the entry count, whose word at `+0x10`
/// (INFO_KEYS) points at the row-id array and whose word at `+0x14`
/// (INFO_VALUES) points at the parallel value array. A zero answer
/// byte means failure and yields -1. Otherwise the keys are scanned
/// from index 0 while signed-less than the count; on a match the
/// value at the same position is returned, or -1 when the count is
/// not positive or nothing matches.
///
/// Original: 0x00530B40 (stdcall, one stack word; the incoming ECX, the
/// virtual-method receiver, is overwritten before any read and ignored).
lf_checker_rt::export!(stdcall, rw_00530B40(key: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x88;
        const HELPER: u32 = 1;
        const INFO_COUNT: usize = 3;
        const INFO_KEYS: usize = 4;
        const INFO_VALUES: usize = 5;
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
        let keys = info[INFO_KEYS];
        let values = info[INFO_VALUES];
        if count <= 0 {
            return NOT_FOUND;
        }
        let mut pos = 0i32;
        let mut found = false;
        while pos < count {
            if rd32(keys.wrapping_add((pos as u32).wrapping_mul(4))) == key {
                found = true;
                break;
            }
            pos += 1;
        }
        if !found {
            return NOT_FOUND;
        }
        rd32(values.wrapping_add((pos as u32).wrapping_mul(4)))
    }
});
