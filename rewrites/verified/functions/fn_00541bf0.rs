// original: 0x00541bf0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_0, player_schema::LeaderboardInfo, 10>::vf6
///
/// leaderboard key search by value: find the position of a key value inside the key table.
///
/// Asks the helper (callee 1, id 0x9e) for the board's key list (count at
/// out offset 12, key-table pointer at offset 16). A helper failure, or a
/// non-positive count (signed check), yields NOT_FOUND. Otherwise scans the
/// key table linearly for `key` and returns its position, or NOT_FOUND.
///
/// Original: thiscall/1, the callee pops 4 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_00541bf0(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x9e;
        const COUNT_SLOT: usize = 3;
        const KEYS_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
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
                return i;
            }
        }
        NOT_FOUND
    }
});
