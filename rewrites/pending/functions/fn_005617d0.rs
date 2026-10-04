// original: 0x005617D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_71, player_schema::LeaderboardInfo, 10>::vf8
/// Classify one leaderboard column value by size.
///
/// Calls the column-table helper (id `0x112`) with a six-word scratch frame;
/// when it answers true in `al`, the value-table pointer is in the frame's
/// word 5 (`TABLE_SLOT`). The `index`-th entry of that table is passed (in
/// `ecx`) to the kind probe, whose answer maps to a size class: 1 gives 4,
/// 2 and 3 give 8, 5 gives 4, 4 gives 0, and anything else (including the
/// probe answering -1, or 0, or more than 5) gives 0. A false answer from the
/// helper also gives 0.
///
/// The original implements the mapping as a five-way jump table over
/// (answer - 1); the `match` below is that table.
///
/// Original: 0x005617D0 (thiscall, one stack word; incoming `this` ignored).
lf_checker_rt::export!(thiscall, rw_005617D0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x112;
        const TABLE_SLOT: usize = 5;
        const SIZE_SMALL: u32 = 4;
        const SIZE_LARGE: u32 = 8;
        let mut frame = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let table = frame[TABLE_SLOT];
        let value = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, value);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => SIZE_SMALL,
            1 | 2 => SIZE_LARGE,
            4 => SIZE_SMALL,
            _ => 0,
        }
    }
});
