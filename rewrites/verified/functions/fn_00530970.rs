// original: 0x00530970 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race35Standard, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard row-id array entry into a width.
///
/// `index` selects the entry. The helper (callee 1) is asked for the
/// row-id table of kind 0x94: it fills a scratch struct whose word
/// at `+0x14` (INFO_IDS) points at the array. A zero answer byte means
/// failure and yields 0. Otherwise the entry is handed to the
/// classifier (callee 2); its answer minus one picks a width from the
/// original's jump table: 0 -> 4, 1 or 2 -> 8, 4 -> 4, anything else
/// (including a -1 answer) -> 0.
///
/// Original: 0x00530970 (stdcall, one stack word; the incoming ECX, the
/// virtual-method receiver, is overwritten before any read and ignored).
lf_checker_rt::export!(stdcall, rw_00530970(index: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x94;
        const HELPER: u32 = 1;
        const CLASSIFY: u32 = 2;
        const INFO_IDS: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok = lf_checker_rt::callee_fastcall!(HELPER, u32, KIND, info.as_mut_ptr() as u32) as u8;
        if ok == 0 {
            return 0;
        }
        let entry = rd32(info[INFO_IDS].wrapping_add(index.wrapping_mul(4)));
        let cls: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, entry);
        if cls == 0xffff_ffff {
            return 0;
        }
        match cls.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
