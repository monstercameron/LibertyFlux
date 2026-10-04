// original: 0x0054AEB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_16, player_schema::LeaderboardInfo, 10>::vf13
/// Value lookup by id for one ranked leaderboard board (this board: id 0xc7).
///
/// Calls the shared fetch step with (`LEADERBOARD_ID`, frame) and, when its
/// answer has a nonzero low byte, scans the id array the fetch step filled in
/// for `id`, returning the value from the parallel value array at the match.
/// Returns -1 when the fetch step reports failure (low byte zero), when the
/// filled count is not positive (compared signed), or when the id is absent.
/// Only the low byte of the fetch answer is tested.
///
/// The fetch step takes its arguments in ECX (board id) and EDX (frame
/// pointer) with no stack arguments, so it is declared fastcall/0; its EDX
/// argument points into this function's own frame and is therefore not part
/// of the compared call (only ECX is).
///
/// Original: stdcall with one stack word; ECX is overwritten before any read,
/// so the method takes no `this` despite living in a vtable. The original
/// re-checks the found index against -1 after a loop that counts up from 0,
/// which can never match and is not repeated here; it also aligns ESP to 8
/// bits on entry, which is unobservable (only the ESP delta is compared) and
/// likewise not repeated.
lf_checker_rt::export!(stdcall, rw_0054aeb0(id: u32) -> u32 {
    unsafe {
        /// Board id this instantiation passes to the fetch step in ECX.
        const LEADERBOARD_ID: u32 = 0xc7;
        /// Frame words the fetch step fills: count, id array, value array.
        const FRAME_COUNT: usize = 3;
        const FRAME_IDS: usize = 4;
        const FRAME_VALUES: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if (answer & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let count = frame.as_ptr().add(FRAME_COUNT).read() as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = frame.as_ptr().add(FRAME_IDS).read() as *const u32;
        let mut i = 0i32;
        loop {
            if i >= count {
                return NOT_FOUND;
            }
            if ids.add(i as usize).read() == id {
                break;
            }
            i = i.wrapping_add(1);
        }
        let values = frame.as_ptr().add(FRAME_VALUES).read();
        (values.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
    }
});
