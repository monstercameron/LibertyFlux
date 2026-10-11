// original: 0x0053A350 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopDrugFactory_BG, player_schema::LeaderboardInfo, 10>::vf2
/// Call the object's second virtual slot. If its 32-bit result equals `expected_value` and `output_slot` is non-null, store the relocated type descriptor through the output pointer and return that pointer. Otherwise return 0. The descriptor differs between template instances.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_0053a350(object_ptr: u32, output_slot: u32, expected_value: u32) -> u32 {
    const TYPE_DESCRIPTOR_VA: u32 = 0x00FD18A4;
    let method_value = lf_checker_rt::callee_thiscall!(1, u32, object_ptr);
    if method_value == expected_value && output_slot != 0 {
        let descriptor = lf_checker_rt::relocated(TYPE_DESCRIPTOR_VA);
        unsafe { core::ptr::write_unaligned(output_slot as *mut u32, descriptor); }
        output_slot
    } else {
        0
    }
}
