// original: 0x0057C800 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_170, player_schema::LeaderboardInfo, 10>::vf6
/// Find a leaderboard key's position in the key list.
///
/// Calls the column-table helper (id `0x182`) with a six-word scratch frame.
/// When it answers true in `al`, the frame holds the key count (word 3,
/// `COUNT_SLOT`, signed: zero or negative gives `NOT_FOUND`) and the
/// key-list pointer (word 4, `KEYS_SLOT`). `key` is searched from element 0
/// and the first matching position wins; an absent key gives `NOT_FOUND` (-1).
///
/// Original: 0x0057C800 (thiscall, one stack word; incoming `this` ignored).
lf_checker_rt::export!(thiscall, rw_0057C800(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x182;
        const COUNT_SLOT: usize = 3;
        const KEYS_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = frame[COUNT_SLOT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = frame[KEYS_SLOT];
        let mut pos = 0u32;
        while (pos as i32) < count {
            let cand = ((keys.wrapping_add(pos.wrapping_mul(4))) as *const u32).read_unaligned();
            if cand == key {
                return pos;
            }
            pos = pos.wrapping_add(1);
        }
        NOT_FOUND
    }
});
