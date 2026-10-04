// original: 0x005311F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race37Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of the leaderboard row-id array by index.
///
/// `index` selects the entry. The helper (callee 1) is asked for the
/// row-id table of kind 0x84: it fills a scratch struct whose word
/// at `+0x10` (INFO_IDS) points at the array. A zero answer byte means
/// failure and yields -1. Otherwise the word at `array[index]` is
/// returned with no bounds check, so a wild index reads whatever the
/// address holds or faults exactly like the original's single load.
///
/// Original: 0x005311F0 (stdcall, one stack word; the incoming ECX, the
/// virtual-method receiver, is overwritten before any read and ignored).
lf_checker_rt::export!(stdcall, rw_005311F0(index: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x84;
        const HELPER: u32 = 1;
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
        let ids = info[INFO_IDS];
        rd32(ids.wrapping_add((index).wrapping_mul(4)))
    }
});
