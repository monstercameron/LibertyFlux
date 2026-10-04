// original: 0x0056d410 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_114, player_schema::LeaderboardInfo, 10>::vf9
/// Rank of `keys[index]` from its class: 0, 1, 2, 3, or NOT_FOUND.
///
/// Fetches the tables for LEADERBOARD_ID and returns NOT_FOUND when the fetch
/// fails. Otherwise classifies `keys[index]` (keys table at frame word 5)
/// through the shared classify step, which takes the key in ECX. A class of
/// -1, class 0, or anything past 5 means NOT_FOUND; classes 1..5 map through
/// [0, 1, 3, -1, 2] (the original's five-entry jump table, re-expressed).
///
/// stdcall with one stack argument; no `this`, no globals, no caller-visible
/// writes.
lf_checker_rt::export!(stdcall, rw_0056d410(index: u32) -> u32 {
    unsafe {
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const BAD_CLASS: u32 = 0xFFFF_FFFF;
        const KEYS_WORD: usize = 5;
        const CLASSIFY_CALLEE: u32 = 2;
        const LEADERBOARD_ID: u32 = 317;
        const FETCH_CALLEE: u32 = 1;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32, LEADERBOARD_ID,
            frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let keys = frame[KEYS_WORD];
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let class = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, key);
        if class == BAD_CLASS {
            return NOT_FOUND;
        }
        let slot = class.wrapping_sub(1);
        if slot > 4 {
            return NOT_FOUND;
        }
        const MAP: [u32; 5] = [0u32, 1, 3, 0xFFFF_FFFF, 2];
        MAP[slot as usize]
    }
});
