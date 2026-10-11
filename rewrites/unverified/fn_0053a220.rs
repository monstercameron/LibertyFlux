// original: 0x0053A220 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopDrugFactory, player_schema::LeaderboardInfo, 10>::vf8
/// Ask the selector-specific lookup helper for an array of type handles, read the selected handle, and query its kind. The five-entry dispatch maps kinds 1 and 5 to 4, kinds 2 and 3 to 8, and kinds 4, invalid, or out-of-range values to 0. A declined first lookup also returns 0. The receiver is not read.
lf_checker_rt::export!(thiscall, rw_0053a220(_receiver: u32, index: u32) -> u32 {
    const SELECTOR: u32 = 0x2;
    const NO_TYPE: u32 = u32::MAX;
    #[repr(C)]
    struct TypeTable {
        _leading: [u32; 5],
        handles: *const u32,
    }
    let mut output = TypeTable { _leading: [0; 5], handles: core::ptr::null() };
    let output_address = &mut output as *mut TypeTable as u32;
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, output_address);
    if helper_result & 0xFF == 0 { return 0; }
    let handle = unsafe { *output.handles.add(index as usize) };
    let type_code = lf_checker_rt::callee_thiscall!(2, u32, handle);
    if type_code == NO_TYPE { return 0; }
    let dispatch = type_code.wrapping_sub(1);
    if dispatch > 4 { return 0; }
    let size = match dispatch {
        0 | 4 => 4,
        1 | 2 => 8,
        3 => 0,
        _ => 0,
    };
    size
});
