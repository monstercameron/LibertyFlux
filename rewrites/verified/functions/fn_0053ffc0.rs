// original: 0x0053ffc0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopSwatAssault_BG_TIME, player_schema::LeaderboardInfo, 10>::vf13
///
/// leaderboard aux lookup by key search: find a key value, return the matching auxiliary word.
///
/// Asks the helper (callee 1, id 0x4e) for the key list (count at out
/// offset 12, key table at 16) and the parallel auxiliary table (pointer at
/// offset 20). A helper failure or a non-positive count (signed check)
/// yields NOT_FOUND. Otherwise scans the key table for `key`; on a hit at
/// position `i` returns the auxiliary table's `i`-th word, else NOT_FOUND.
/// (The original re-checks the hit index against -1 before the second
/// lookup; a loop index can never be -1, so the check is kept for fidelity
/// but never fires.)
///
/// Original: thiscall/1, the callee pops 4 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_0053ffc0(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x4e;
        const COUNT_SLOT: usize = 3;
        const KEYS_SLOT: usize = 4;
        const AUX_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let (count, keys) = (out[COUNT_SLOT], out[KEYS_SLOT]);
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        for i in 0..count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == key {
                if i == NOT_FOUND {
                    return NOT_FOUND;
                }
                return rd32(out[AUX_SLOT].wrapping_add(i.wrapping_mul(4)));
            }
        }
        NOT_FOUND
    }
});
