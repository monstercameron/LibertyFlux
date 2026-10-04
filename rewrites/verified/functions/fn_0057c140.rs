// original: 0x0057C140 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_169, player_schema::LeaderboardInfo, 10>::vf12
/// Find a leaderboard key's position in the key list.
///
/// Calls the column-table helper (id `0x181`) with a six-word scratch frame.
/// When it answers true in `al`, the frame holds the key count (word 1,
/// `COUNT_SLOT`), the key-list pointer (word 2, `KEYS_SLOT`) and the
/// value-table pointer (word 5, `TABLE_SLOT`). The `index`-th value-table
/// entry is the key to find; when that key is -1 the result is `NOT_FOUND`
/// without searching. Otherwise the key list is scanned from element 0 while
/// the position is below the count (unsigned), and the first match wins.
/// No match, a zero count, or a false helper answer all give `NOT_FOUND` (-1).
///
/// Original: 0x0057C140 (thiscall, one stack word; incoming `this` ignored).
lf_checker_rt::export!(thiscall, rw_0057C140(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x181;
        const COUNT_SLOT: usize = 1;
        const KEYS_SLOT: usize = 2;
        const TABLE_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let table = frame[TABLE_SLOT];
        let key = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = frame[COUNT_SLOT];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = frame[KEYS_SLOT];
        let mut pos = 0u32;
        while pos < count {
            let cand = ((keys.wrapping_add(pos.wrapping_mul(4))) as *const u32).read_unaligned();
            if cand == key {
                return pos;
            }
            pos = pos.wrapping_add(1);
        }
        NOT_FOUND
    }
});
