// original: 0x00539D90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopBombBase_BG_TIME, player_schema::LeaderboardInfo, 10>::vf7
/// Ask the selector-specific lookup helper for an array of 32-bit values and return `values[index]`. If the helper declines, return `u32::MAX`. The receiver is not read.
lf_checker_rt::export!(thiscall, rw_00539d90(_receiver: u32, index: u32) -> u32 {
    const NO_MATCH: u32 = u32::MAX;
    const SELECTOR: u32 = 0x31;
    #[repr(C)]
    struct IndexedValues {
        _leading: [u32; 4],
        values: *const u32,
    }
    let mut output = IndexedValues { _leading: [0; 4], values: core::ptr::null() };
    let output_address = &mut output as *mut IndexedValues as u32;
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, output_address);
    if helper_result & 0xFF == 0 { return NO_MATCH; }
    let value = unsafe { *output.values.add(index as usize) };
    value
});
