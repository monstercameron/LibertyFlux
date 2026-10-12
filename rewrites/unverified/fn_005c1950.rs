// original: 0x005C1950 load_control_map_if_initialized

/// Load the control map after optionally preparing the current game state.
/// The optional preparation receives the current state pointer from its
/// global slot; loading always uses the relocated control-map object and a
/// zero flag. Both calls are thiscall operations, and the final helper's
/// return value is the value returned by this cdecl wrapper.
lf_checker_rt::export!(cdecl, rw_005c1950() -> u32 {
    const GAME_STATE_POINTER_VA: u32 = 0x01BB5624;
    const CONTROL_MAP_OBJECT_VA: u32 = 0x019D2DC0;
    const PREPARE_GAME_STATE: u32 = 1;
    const LOAD_CONTROL_MAP: u32 = 2;

    let state = unsafe { lf_checker_rt::global::<u32>(GAME_STATE_POINTER_VA).read_unaligned() };
    if state != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(PREPARE_GAME_STATE, u32, state);
    }
    lf_checker_rt::callee_thiscall!(
        LOAD_CONTROL_MAP,
        u32,
        lf_checker_rt::relocated(CONTROL_MAP_OBJECT_VA),
        0
    )
});
