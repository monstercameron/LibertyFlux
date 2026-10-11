// original: 0x00D2AAD0 CTaskComplexMoveFollowNavMeshRoute::CTaskComplexMoveFollowNavMeshRoute_2

/// Initialize the alternate move-follow-route constructor from a raw initial
/// weight word, a three-word route position, three additional float-shaped
/// words, a state word, and three flag bytes. `task` is the implicit thiscall
/// receiver. The route position is copied to task offsets `0x20..0x2c` and
/// `0x30..0x3c`; weights go to `0x50..0x60`, the initial state to `0x9c`, the
/// derived flags to `0xd8`, and the return state to `0xe0`. The remaining
/// listed state words are cleared. The low bits of the flag bytes map to bits
/// 4, 5, and 7; bit 1 is always set and other retained bits come from the
/// existing flag word. The initial weight is forwarded bit-for-bit to the
/// base constructor. Float-shaped values are copied as raw words, preserving
/// signed zero and NaN payloads. The original uses thiscall with ten stack
/// words and returns the receiver in EAX.
lf_checker_rt::export!(thiscall, rw_00d2aad0(
    task: u32,
    initial_weight: u32,
    route_position: u32,
    weight_primary: u32,
    weight_secondary: u32,
    initial_state: u32,
    task_flags_low: u32,
    task_flags_middle: u32,
    return_state: u32,
    weight_quaternary: u32,
    task_flags_high: u32,
) -> u32 {
    unsafe {
        const ROUTE_CLASS_VTABLE: u32 = 0x00EE_1F94;
        const ROUTE_CLASS_STATE_VTABLE: u32 = 0x00EE_1FEC;
        const TASK_ROUTE_POSITION_A: u32 = 0x20;
        const TASK_ROUTE_POSITION_B: u32 = 0x30;
        const TASK_WEIGHT_PRIMARY: u32 = 0x50;
        const TASK_WEIGHT_SECONDARY: u32 = 0x54;
        const TASK_WEIGHT_TERTIARY: u32 = 0x58;
        const TASK_WEIGHT_QUATERNARY: u32 = 0x5C;
        const TASK_INITIAL_STATE: u32 = 0x9C;
        const TASK_FLAGS: u32 = 0xD8;
        const TASK_RETURN_STATE: u32 = 0xE0;
        const TASK_FLAGS_KEEP_MASK: u32 = 0xFFFF_FF4F;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        let _base_result: u32 =
            lf_checker_rt::callee_thiscall!(1, u32, task, initial_weight);

        write_u32(task, lf_checker_rt::relocated(ROUTE_CLASS_VTABLE));
        write_u32(
            task + 0x14,
            lf_checker_rt::relocated(ROUTE_CLASS_STATE_VTABLE),
        );

        let position_x = read_u32(route_position);
        let position_y = read_u32(route_position + 4);
        let position_z = read_u32(route_position + 8);
        write_u32(task + TASK_ROUTE_POSITION_A, position_x);
        write_u32(task + TASK_ROUTE_POSITION_A + 4, position_y);
        write_u32(task + TASK_ROUTE_POSITION_A + 8, position_z);
        write_u32(task + TASK_ROUTE_POSITION_B, position_x);
        write_u32(task + TASK_ROUTE_POSITION_B + 4, position_y);
        write_u32(task + TASK_ROUTE_POSITION_B + 8, position_z);

        write_u32(task + TASK_WEIGHT_PRIMARY, weight_primary);
        write_u32(task + TASK_WEIGHT_SECONDARY, weight_secondary);
        write_u32(task + TASK_WEIGHT_TERTIARY, weight_primary);
        write_u32(task + TASK_WEIGHT_QUATERNARY, weight_quaternary);
        write_u32(task + TASK_INITIAL_STATE, initial_state);

        for offset in [0xA0, 0xA4, 0xAC, 0xB0, 0xB8, 0xBC, 0xC4, 0xC8] {
            write_u32(task + offset, 0);
        }
        write_u16(task + 0xA8, 0);
        write_u16(task + 0xB4, 0);
        write_u16(task + 0xC0, 0);
        write_u16(task + 0xCC, 0);

        let prior_flags = read_u32(task + TASK_FLAGS);
        let route_flags = ((task_flags_high & 1) << 7)
            | ((task_flags_middle & 1) << 5)
            | ((task_flags_low & 1) << 4);
        write_u32(
            task + TASK_FLAGS,
            (prior_flags & TASK_FLAGS_KEEP_MASK) | route_flags | 2,
        );
        write_u32(task + TASK_RETURN_STATE, return_state);

        let _setup_result: u32 = lf_checker_rt::callee_thiscall!(2, u32, task);
        task
    }
});
