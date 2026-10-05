// original: 0x00541530 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_TeamVip, player_schema::LeaderboardInfo, 10>::vf12
///
/// leaderboard key search by row index: find the position of one row's key inside the key table.
///
/// Asks the helper (callee 1, id 0x4b) for the board's key list (count at
/// out offset 4, key-table pointer at offset 8) and the row table (pointer at
/// offset 20). Looks up the key of row `index` in the row table; a missing
/// row (key NOT_FOUND) or a helper failure yields NOT_FOUND. Then scans the
/// key table linearly (unsigned bound) for that key and returns its position,
/// or NOT_FOUND when the table is empty or holds no such key.
///
/// Original: thiscall/1, the callee pops 4 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_00541530(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x4b;
        const COUNT_SLOT: usize = 1;
        const KEYS_SLOT: usize = 2;
        const ROWS_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let wanted = rd32(out[ROWS_SLOT].wrapping_add(index.wrapping_mul(4)));
        if wanted == NOT_FOUND {
            return NOT_FOUND;
        }
        let (count, keys) = (out[COUNT_SLOT], out[KEYS_SLOT]);
        if count == 0 {
            return NOT_FOUND;
        }
        for i in 0..count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == wanted {
                return i;
            }
        }
        NOT_FOUND
    }
});
