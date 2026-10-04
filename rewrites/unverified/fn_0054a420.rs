// original: 0x0054A420 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_13, player_schema::LeaderboardInfo, 10>::vf8
/// Column-width query for one ranked leaderboard board (this board: id 0xbf).
///
/// Calls the shared fetch step with (`LEADERBOARD_ID`, frame) and, when its
/// answer has a nonzero low byte, loads the key at `index` from the key table
/// the fetch step filled in, classifies it through the shared classify step
/// (key passed in ECX), and maps the class to a width: class 1 takes 4,
/// classes 2-3 take 8, class 5 takes 4, anything else takes 0. A failed fetch
/// or a class of -1 takes 0. Only the low byte of the fetch answer is tested.
///
/// The class map above is this instantiation's jump table, read from the
/// image and asserted by the generator; every instantiation in this batch
/// carries the same map.
///
/// Original: stdcall with one stack word; ECX is overwritten before any read,
/// so the method takes no `this` despite living in a vtable.
lf_checker_rt::export!(stdcall, rw_0054a420(index: u32) -> u32 {
    unsafe {
        /// Board id this instantiation passes to the fetch step in ECX.
        const LEADERBOARD_ID: u32 = 0xbf;
        /// Frame word the fetch step fills with the key-table pointer.
        const FRAME_KEYS: usize = 5;
        const FAILED_CLASS: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if (answer & 0xFF) == 0 {
            return 0;
        }
        let keys = frame.as_ptr().add(FRAME_KEYS).read();
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let class = lf_checker_rt::callee_thiscall!(2, u32, key);
        if class == FAILED_CLASS {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
