// original: 0x00DCA7E0 CTaskComplexCarReactToVehicleCollision::vf19

/// Creates the next collision-response subtask for a player ped.
///
/// `this` is the collision task in ECX and `player` is the one stack
/// argument. The routine proceeds only when the player's task flags include
/// bit two, both stored entity pointers are non-null and distinct, and the
/// player's byte at `+0x219` is zero. It records the returned task type at
/// task offset `+0x14`, selects a response type using an unsigned range check,
/// and returns the response constructor's EAX value. Types two through nine
/// are handled by the switch; type two also selects between two constants
/// using the task's word at `+0x20`, and all other values use the fallback.
lf_checker_rt::export!(thiscall, rw_00dca7e0(this: u32, player: u32) -> u32 {
    unsafe {
        const PLAYER_FLAGS: u32 = 0x26c;
        const PLAYER_GATE: u32 = 1 << 2;
        const VEHICLE_A: u32 = 0x18;
        const VEHICLE_B: u32 = 0x1c;
        const PLAYER_MODE: u32 = 0x219;
        const STORED_TASK_TYPE: u32 = 0x14;
        const TASK_VARIANT: u32 = 0x20;
        const QUERY_NEXT_TYPE: u32 = 1;
        const CREATE_RESPONSE: u32 = 2;
        const TYPE_TWO: u32 = 2;
        const TYPE_NINE: u32 = 9;
        const TASK_KIND_A: u32 = 0x76c;
        const TASK_KIND_B: u32 = 0x516;
        const TASK_KIND_MIDDLE: u32 = 0x149;
        const TASK_KIND_LAST: u32 = 0x2d6;
        const TASK_KIND_FALLBACK: u32 = 0x16c;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let player_flags = read_u32(player.wrapping_add(PLAYER_FLAGS));
        let first_vehicle = read_u32(this.wrapping_add(VEHICLE_A));
        let second_vehicle = read_u32(this.wrapping_add(VEHICLE_B));
        let player_mode = (player.wrapping_add(PLAYER_MODE) as *const u8).read();
        if player_flags & PLAYER_GATE == 0
            || first_vehicle == 0
            || second_vehicle == 0
            || first_vehicle == second_vehicle
            || player_mode != 0
        {
            return 0;
        }

        let task_type = lf_checker_rt::callee_thiscall!(QUERY_NEXT_TYPE, u32, this, player);
        write_u32(this.wrapping_add(STORED_TASK_TYPE), task_type);

        let task_kind = task_type.wrapping_sub(TYPE_TWO);
        if task_kind > TYPE_NINE - TYPE_TWO {
            return lf_checker_rt::callee_thiscall!(
                CREATE_RESPONSE,
                u32,
                this,
                TASK_KIND_FALLBACK,
                player
            );
        }

        let response_kind = match task_type {
            TYPE_TWO if read_u32(this.wrapping_add(TASK_VARIANT)) != 0 => TASK_KIND_A,
            TYPE_TWO => TASK_KIND_B,
            TYPE_NINE => TASK_KIND_LAST,
            _ => TASK_KIND_MIDDLE,
        };
        lf_checker_rt::callee_thiscall!(CREATE_RESPONSE, u32, this, response_kind, player)
    }
});
