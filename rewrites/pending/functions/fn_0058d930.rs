// original: 0x0058D930 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_233, player_schema::LeaderboardInfo, 10>::vf12
/// Leaderboard reverse lookup: index whose id equals element INDEX of the
/// key column, or -1.
///
/// Arguments: INDEX is the single stack word. ECX is ignored. Calls the
/// shared lookup helper (fastcall: ECX = board id 0x1C1, EDX = frame
/// buffer; AL tested). On success COUNT is at buffer+0x04, the id array at
/// buffer+0x08 and the key column at buffer+0x14. Loads key = keys[INDEX]
/// with no bounds check (-1 key returns -1), then scans the id array with an
/// UNSIGNED loop (`test/je` entry so only exactly zero skips, `cmp/jb`
/// step), so a large count scans far. Returns the first matching index,
/// else 0xFFFFFFFF. AL == 0 returns 0xFFFFFFFF.
/// Original: 0x0058D930 (stdcall, one stack word; ignores ECX).

lf_checker_rt::export!(stdcall, rw_0058D930(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1C1;
        const COUNT_OFF: u32 = 0x04;
        const IDS_OFF: u32 = 0x08;
        const KEYS_OFF: u32 = 0x14;
        const NONE: u32 = 0xFFFFFFFF;
        let mut out = [0u32; 8];
        let r: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if (r & 0xFF) == 0 {
            return NONE;
        }
        let keys = out[(KEYS_OFF / 4) as usize];
        let key = ((keys.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        if key == NONE {
            return NONE;
        }
        let count = out[(COUNT_OFF / 4) as usize];
        if count == 0 {
            return NONE;
        }
        let ids = out[(IDS_OFF / 4) as usize];
        let mut i: u32 = 0;
        loop {
            let v = ((ids.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NONE;
            }
        }
    }
});
