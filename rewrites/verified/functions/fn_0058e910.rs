// original: 0x0058E910 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_236, player_schema::LeaderboardInfo, 10>::vf7
/// Leaderboard key read: return the key at position `index`.
/// Calls the leaderboard helper (id 0x1c4) with a five-word scratch
/// record; on success the helper leaves the key-table pointer at record
/// word 4 (`+0x10`). Returns the table word at `index`, or 0xFFFFFFFF when
/// the helper fails. The index is scaled by four with wraparound and is not
/// range-checked.
/// Original: 0x0058E910 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058e910(index: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c4;
        const KEYS_WORD: usize = 4;
        const FAIL: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 5];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return FAIL;
        }
        rd32(info[KEYS_WORD].wrapping_add(index.wrapping_mul(4)))
    }
});
