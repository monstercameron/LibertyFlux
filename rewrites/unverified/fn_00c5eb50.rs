// original: 0x00C5EB50 set_driveby_seat_door

/// Selects a latched-door action from a character's vehicle, seat class,
/// model flags, and current door value. A missing door returns zero. The
/// special model flag combined with seat code 1 or 3 uses the unit-valued
/// door action. Vehicle state 4 also selects it for other seat codes. In
/// remaining cases, a positive ordered door value uses the zero-valued
/// action. A NaN does not take the positive action.
#[unsafe(no_mangle)]
pub extern "stdcall" fn rw_00c5eb50(_unused: u32, ped: u32) -> u32 {
    unsafe {
        const PED_VEHICLE_OFFSET: u32 = 0x0b30;
        const VEHICLE_MODEL_INDEX: u32 = 0x2e;
        const MODEL_TABLE_VA: u32 = 0x01295cd8;
        const MODEL_FLAGS_OFFSET: u32 = 0x94;
        const MODEL_SPECIAL_SHIFT: u32 = 5;
        const VEHICLE_STATE_OFFSET: u32 = 0x1304;
        const DOOR_VALUE_OFFSET: u32 = 0x1c;
        const SEAT_SPECIAL_A: u32 = 1;
        const SEAT_SPECIAL_B: u32 = 3;
        const DOOR_ACTION_UNIT: u32 = 0x3f80_0000;
        const DOOR_ACTION_UNIT_FLAG: u32 = 0x0200;
        const DOOR_ACTION_OPEN_FLAG: u32 = 0x020200;

        let vehicle = (ped.wrapping_add(PED_VEHICLE_OFFSET) as *const u32)
            .read_unaligned();
        let seat_code: u32 = lf_checker_rt::callee_thiscall!(1, u32, vehicle, ped);
        let model_row: u32 = lf_checker_rt::callee_thiscall!(2, u32, vehicle, seat_code);
        let door: u32 = lf_checker_rt::callee_thiscall!(3, u32, vehicle, model_row);
        if door == 0 {
            return 0;
        }

        let model_index = (vehicle.wrapping_add(VEHICLE_MODEL_INDEX) as *const i16)
            .read_unaligned() as isize;
        let model_info_table = lf_checker_rt::global::<u32>(MODEL_TABLE_VA);
        let model_info = model_info_table.offset(model_index).read_unaligned();
        let model_flags = (model_info.wrapping_add(MODEL_FLAGS_OFFSET) as *const u32)
            .read_unaligned();
        let special_seat = seat_code == SEAT_SPECIAL_A || seat_code == SEAT_SPECIAL_B;
        let special_model_seat = (model_flags >> MODEL_SPECIAL_SHIFT) & 1 != 0
            && special_seat;
        let vehicle_state = (vehicle.wrapping_add(VEHICLE_STATE_OFFSET) as *const u32)
            .read_unaligned();

        if (vehicle_state == 4 && !special_seat) || special_model_seat {
            return lf_checker_rt::callee_thiscall!(
                4,
                u32,
                door,
                DOOR_ACTION_UNIT,
                DOOR_ACTION_UNIT_FLAG
            );
        }

        let door_value = f32::from_bits(
            (door.wrapping_add(DOOR_VALUE_OFFSET) as *const u32).read_unaligned(),
        );
        if !(door_value > 0.0) {
            return door;
        }
        lf_checker_rt::callee_thiscall!(
            4,
            u32,
            door,
            0,
            DOOR_ACTION_OPEN_FLAG
        )
    }
}
