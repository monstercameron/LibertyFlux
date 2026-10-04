// original: 0x0058DE00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_234, player_schema::LeaderboardInfo, 10>::vf13
/// Leaderboard joined read: value-column element at the row whose id is KEY.
///
/// Arguments: KEY is the single stack word. ECX is ignored. Calls the
/// shared lookup helper (fastcall: ECX = board id 0x1C2, EDX = frame
/// buffer; AL tested). On success COUNT is at buffer+0x0c (signed: zero or
/// negative returns -1), the id array at buffer+0x10 and the value column
/// at buffer+0x14. Scans ids with a signed loop for KEY; on a hit at row i
/// returns values[i]. The original then re-checks i against -1, which is
/// unreachable (i counts up from 0) and kept here for fidelity. A miss or
/// AL == 0 returns 0xFFFFFFFF.
/// Original: 0x0058DE00 (stdcall, one stack word; ignores ECX).

lf_checker_rt::export!(stdcall, rw_0058DE00(key: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1C2;
        const COUNT_OFF: u32 = 0x0c;
        const IDS_OFF: u32 = 0x10;
        const VALS_OFF: u32 = 0x14;
        const NONE: u32 = 0xFFFFFFFF;
        let mut out = [0u32; 8];
        let r: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if (r & 0xFF) == 0 {
            return NONE;
        }
        let count = out[(COUNT_OFF / 4) as usize] as i32;
        if count <= 0 {
            return NONE;
        }
        let ids = out[(IDS_OFF / 4) as usize];
        let mut i: i32 = 0;
        loop {
            let v = ((ids.wrapping_add((i as u32).wrapping_mul(4))) as *const u32).read_unaligned();
            if v == key {
                break;
            }
            i += 1;
            if i >= count {
                return NONE;
            }
        }
        if i == -1 {
            return NONE;
        }
        let vals = out[(VALS_OFF / 4) as usize];
        ((vals.wrapping_add((i as u32).wrapping_mul(4))) as *const u32).read_unaligned()
    }
});
