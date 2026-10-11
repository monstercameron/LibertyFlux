// original: 0x0053FDB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopDrugFactory_BG_TIME, player_schema::LeaderboardInfo, 10>::vf7

/// Ask the selector-specific helper to fill a six-word record whose fifth word points to a value array. Return the 32-bit value at `index`, or `u32::MAX` if the helper declines. The original performs no bounds check. The receiver is unused.
lf_checker_rt::export!(thiscall, rw_0053fdb0(_this: u32, index: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0x4D;
        const NO_MATCH: u32 = u32::MAX;
#[repr(C)]
    struct IndexedValues { _words_0_3: [u32; 4], values: u32, _word_5: u32 }
    let mut info = IndexedValues { _words_0_3: [0; 4], values: 0, _word_5: 0 };
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, &mut info as *mut _ as u32);
    if helper_result as u8 == 0 { return NO_MATCH; }
        let values = info.values as *const u32;
        let value = values.wrapping_add(index as usize).read_unaligned();
        value
    }
});
