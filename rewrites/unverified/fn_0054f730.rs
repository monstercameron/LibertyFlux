// original: 0x0054f730 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_5, player_schema::LeaderboardInfo, 10>::vf8
/// Classify one leaderboard slot into a small size code.
///
/// Asks the table-fill callee for leaderboard `LEADERBOARD_ID`, which writes
/// an info block holding the `SLOTS` array pointer at `+0x14`. Returns 0 when
/// the fill failed. Otherwise reads `SLOTS[idx]`, asks the kind callee for
/// its kind, and maps the kind: -1 (unknown) and anything past 5 map to 0;
/// kinds 1 and 5 map to 4; kinds 2 and 3 map to 8; kind 4 maps to 0.
///
/// Original: one stack word (`idx`), callee pops 4; incoming `ecx` is
/// overwritten before use, so the call behaves as stdcall. The shape is a
/// range-checked jump table of five arms over `kind - 1`.
lf_checker_rt::export!(stdcall, rw_0054f730(idx: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0xC2;
        const FILL_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const SLOTS: usize = 5;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FILL_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let slot = rd32(info[SLOTS].wrapping_add(idx.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, slot);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
