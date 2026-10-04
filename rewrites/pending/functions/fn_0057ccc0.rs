// original: 0x0057CCC0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_171, player_schema::LeaderboardInfo, 10>::vf7
/// Look up one leaderboard column value by position.
///
/// Calls the column-table helper (id `0x183`) with a six-word scratch frame,
/// which answers whether the table is available in `al` and, when it is,
/// leaves the value-table pointer in the frame's word 4 (`TABLE_SLOT`). The
/// result is the `index`-th 32-bit entry of that table.
///
/// `index` is used as-is (`table + index*4` with wrapping arithmetic); a wild
/// index faults exactly like the original. When the helper answers false the
/// result is `NOT_FOUND` (-1) without touching the table.
///
/// Original: 0x0057CCC0 (thiscall, one stack word; the incoming `this` is
/// ignored: the original overwrites `ecx` with the table id first thing).
lf_checker_rt::export!(thiscall, rw_0057CCC0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x183;
        const TABLE_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let table = frame[TABLE_SLOT];
        ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});
