// original: 0x00589ee0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_219, player_schema::LeaderboardInfo, 10>::vf8

/// Column-width class for one ranked episodic-race leaderboard.
///
/// Asks the leaderboard helper (callee 1) for the column list of leaderboard
/// id 0x1b3, classifies element `index` of the returned array through the
/// column classifier (callee 2, argument in ECX), and maps the class to a
/// width: class 1 -> 4, classes 2-3 -> 8, class 5 -> 4, anything else
/// (including classifier failure, reported as -1) -> 0. Helper failure also
/// yields 0. The index is not bounds-checked.
///
/// The helper takes the leaderboard id in ECX and an out-buffer in EDX and
/// answers in AL; on success it fills the array pointer at buffer byte 20.
/// This method ignores its `this` pointer and takes one stack argument
/// (`index`). Original is stdcall (the callee pops 4 bytes); the class map is the original's
/// five-entry jump table, whose entries pair up as (4, 8, 8, 0, 4).

lf_checker_rt::export!(stdcall, rw_00589ee0(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LEADERBOARD_ID: u32 = 0x1b3;
        const HELPER: u32 = 1;
        const CLASSIFY: u32 = 2;
        const ARRAY_WORD: usize = 5;
        const CLASS_FAILED: u32 = 0xffff_ffff;

        let mut info = [0u32; 6];
        info[ARRAY_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let cell = info[ARRAY_WORD].wrapping_add(index.wrapping_mul(4));
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, rd32(cell));
        if class == CLASS_FAILED {
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
