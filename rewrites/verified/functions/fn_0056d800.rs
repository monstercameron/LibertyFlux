// original: 0x0056d800 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_115, player_schema::LeaderboardInfo, 10>::vf8
/// Width class of `keys[index]`: 4, 8, or NONE (0).
///
/// Fetches the tables for LEADERBOARD_ID and returns NONE when the fetch
/// fails. Otherwise classifies `keys[index]` (keys table at frame word 5)
/// through the shared classify step, which takes the key in ECX. A class of
/// -1, class 0, or anything past 5 means NONE; classes 1..5 map through
/// [4, 8, 8, 0, 4] (the original's five-entry jump table, re-expressed).
///
/// stdcall with one stack argument; no `this`, no globals, no caller-visible
/// writes.
lf_checker_rt::export!(stdcall, rw_0056d800(index: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0;
        const BAD_CLASS: u32 = 0xFFFF_FFFF;
        const KEYS_WORD: usize = 5;
        const CLASSIFY_CALLEE: u32 = 2;
        const LEADERBOARD_ID: u32 = 318;
        const FETCH_CALLEE: u32 = 1;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32, LEADERBOARD_ID,
            frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NONE;
        }
        let keys = frame[KEYS_WORD];
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let class = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, key);
        if class == BAD_CLASS {
            return NONE;
        }
        let slot = class.wrapping_sub(1);
        if slot > 4 {
            return NONE;
        }
        const MAP: [u32; 5] = [4u32, 8, 8, 0, 4];
        MAP[slot as usize]
    }
});
