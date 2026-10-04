// original: 0x0058C150 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_227, player_schema::LeaderboardInfo, 10>::vf6
/// Leaderboard column lookup: index of KEY in this board's id array, or -1.
///
/// Arguments: KEY is the single stack word (id to find). The object pointer
/// in ECX is ignored: the body overwrites ECX with the board id before use.
/// Calls the shared lookup helper (fastcall: ECX = board id 0x1BB, EDX =
/// address of an 8-word frame buffer) and tests only AL of its answer.
/// On success the helper leaves COUNT at buffer+0x0c and the array pointer at
/// buffer+0x10; both are then scanned with a signed loop (`test/jle` entry,
/// `cmp/jl` step), so a zero or negative count returns -1 without reading.
/// Returns the first index whose element equals KEY, else 0xFFFFFFFF.
/// Original: 0x0058C150 (stdcall, one stack word; ignores ECX).

lf_checker_rt::export!(stdcall, rw_0058C150(key: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1BB;
        const COUNT_OFF: u32 = 0x0c;
        const ARRAY_OFF: u32 = 0x10;
        const NONE: u32 = 0xFFFFFFFF;
        let mut out = [0u32; 8];
        let r: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if (r & 0xFF) == 0 {
            return NONE;
        }
        let count = out[(COUNT_OFF / 4) as usize] as i32;
        let arr = out[(ARRAY_OFF / 4) as usize];
        let mut i: i32 = 0;
        while i < count {
            let v = ((arr.wrapping_add((i as u32).wrapping_mul(4))) as *const u32).read_unaligned();
            if v == key {
                return i as u32;
            }
            i += 1;
        }
        NONE
    }
});
