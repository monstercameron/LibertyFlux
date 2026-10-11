// original: 0x009B62E0 script_forward_current_action_code

/// Forward the current action object's virtual result. The stdcall routine
/// consumes one unused stack argument, asks the helper for an object using the
/// address of the manager global as ECX, and returns `u32::MAX` when no object
/// exists. Otherwise it calls the object's virtual slot at byte offset `0x28`
/// with the object in ECX and returns that full 32-bit result.
lf_checker_rt::export!(stdcall, rw_009b62e0(_unused: u32) -> u32 {
    const MANAGER_GLOBAL: u32 = 0x0103_e498;
    const GET_ACTION: u32 = 1;
    const FORWARD_RESULT: u32 = 2;

    let manager_slot = lf_checker_rt::relocated(MANAGER_GLOBAL);
    let action = lf_checker_rt::callee_thiscall!(GET_ACTION, u32, manager_slot);
    if action == 0 {
        u32::MAX
    } else {
        lf_checker_rt::callee_thiscall!(FORWARD_RESULT, u32, action)
    }
});
