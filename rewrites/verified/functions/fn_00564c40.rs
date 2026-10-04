// original: 0x00564c40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_83, player_schema::LeaderboardInfo, 10>::vf8
/// Look up leaderboard column 0x11e, classify element `index` of its value
/// array, and return the element width in bytes (4 or 8), or 0.
///
/// Calls the shared column lookup with id `LOOKUP_ID` (word 5 of the
/// six-word out-struct receives the value-array pointer), then the element
/// classifier on the selected value. Classifier results map to widths 1->4,
/// 2->8, 3->8, 5->4; a failed lookup, a classifier result of -1, and
/// anything else (0, 4, above 5) all yield 0.
///
/// Original: stdcall of one stack word (the index); incoming ECX ignored.
lf_checker_rt::export!(stdcall, rw_00564c40(index: u32) -> u32 {
    unsafe {
        /// Column id selected by this instantiation.
        const LOOKUP_ID: u32 = 0x11e;
        const OUT_VALUES: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 6];
        let answered: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LOOKUP_ID, out.as_mut_ptr() as u32);
        if answered & 0xFF == 0 {
            return 0;
        }
        let value = rd32(out[OUT_VALUES].wrapping_add(index.wrapping_mul(4)));
        // The classifier takes its input in ECX only; EDX is scratch.
        let class: u32 = lf_checker_rt::callee_fastcall!(2, u32, value, 0);
        match class {
            1 => 4,
            2 | 3 => 8,
            5 => 4,
            _ => 0,
        }
    }
});
