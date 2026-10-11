// original: 0x0053FE60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopDrugFactory_BG_TIME, player_schema::LeaderboardInfo, 10>::vf9

/// Ask the selector-specific helper for a handle array, read the handle at `index`, and query its kind. The five-way dispatch maps kinds 1 through 5 to 0, 1, 3, `u32::MAX`, and 2. A declined lookup, sentinel kind, or out-of-range kind returns `u32::MAX`. The receiver is unused.
lf_checker_rt::export!(thiscall, rw_0053fe60(_this: u32, index: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0x4D;
        const NO_MATCH: u32 = u32::MAX;
#[repr(C)]
    struct TypeHandles { _words_0_4: [u32; 5], handles: u32 }
    let mut info = TypeHandles { _words_0_4: [0; 5], handles: 0 };
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, &mut info as *mut _ as u32);
    if helper_result as u8 == 0 { return NO_MATCH; }
        let handle = (info.handles as *const u32).wrapping_add(index as usize).read_unaligned();
        let kind_code = lf_checker_rt::callee_thiscall!(2, u32, handle);
        if kind_code == NO_MATCH { return NO_MATCH; }
        let dispatch = kind_code.wrapping_sub(1);
        if dispatch > 4 { return NO_MATCH; }
        match dispatch { 0 => 0, 1 => 1, 2 => 3, 3 => NO_MATCH, _ => 2 }
    }
});
