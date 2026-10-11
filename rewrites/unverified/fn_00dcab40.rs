// original: 0x00DCAB40 CTaskComplexCarReactToVehicleCollision::vf18

/// Selects a collision-response task kind from the current subtask and player.
///
/// `this` is the collision task in ECX and `context` is its one stack
/// argument. The current subtask's virtual type is queried first. Types other
/// than `0x76c` construct kind `0x516`. For type `0x76c`, a missing vehicle
/// selects `0x516`; otherwise the player's flag at `+0x26c` selects `0x2de`
/// when bit two is clear and `0x516` when it is set. The constructor's EAX
/// value is returned.
lf_checker_rt::export!(thiscall, rw_00dcab40(this: u32, context: u32) -> u32 {
    unsafe {
        const CURRENT_SUBTASK: u32 = 0x08;
        const VEHICLE_A: u32 = 0x18;
        const PLAYER_FLAGS: u32 = 0x26c;
        const PLAYER_GATE: u32 = 1 << 2;
        const EXPECTED_TYPE: u32 = 0x76c;
        const DEFAULT_TASK_KIND: u32 = 0x516;
        const NO_VEHICLE_TASK_KIND: u32 = 0x516;
        const PLAYER_BLOCKED_TASK_KIND: u32 = 0x2de;
        const GET_SUBTASK_TYPE: u32 = 1;
        const CREATE_FALLBACK: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let subtask = read_u32(this.wrapping_add(CURRENT_SUBTASK));
        let subtask_type = lf_checker_rt::callee_thiscall!(GET_SUBTASK_TYPE, u32, subtask);
        let response_kind = if subtask_type != EXPECTED_TYPE {
            DEFAULT_TASK_KIND
        } else if read_u32(this.wrapping_add(VEHICLE_A)) == 0 {
            NO_VEHICLE_TASK_KIND
        } else if read_u32(context.wrapping_add(PLAYER_FLAGS)) & PLAYER_GATE == 0 {
            PLAYER_BLOCKED_TASK_KIND
        } else {
            DEFAULT_TASK_KIND
        };

        lf_checker_rt::callee_thiscall!(CREATE_FALLBACK, u32, this, response_kind, context)
    }
});
