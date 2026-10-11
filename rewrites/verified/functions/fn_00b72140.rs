// original: 0x00B72140 CTaskSimpleCarOpenDoorFromOutside::vf17
/// Performs the outside-door opening task. The task and vehicle objects use
/// 32-bit game layouts at named offsets; the signed model index selects one
/// model record from the executable's table. The one stack argument is the
/// ped object. Existing task state is preserved unless the helper path updates
/// it, and a missing vehicle or nonzero mode returns success immediately.
lf_checker_rt::export!(thiscall, rw_00b72140(task: u32, ped: u32) -> u8 {
    const TASK_CAR: u32 = 0x24;
    const TASK_MODE: u32 = 0x14;
    const TASK_SUBTASK: u32 = 0x18;
    const TASK_PED: u32 = 0x28;
    const TASK_STATE: u32 = 0x30;
    const TASK_FLAG: u32 = 0x2C;
    const TASK_COMPLETE: u32 = 0x2D;
    const CAR_STATE: u32 = 0xF50;
    const CAR_STATE_READY: u32 = 0x219;
    const CAR_STATE_FLAGS: u32 = 0x26C;
    const CAR_MODEL_INDEX: u32 = 0x2E;
    const CAR_MODEL_STATUS: u32 = 0xC4;
    const CAR_FALLBACK_STATUS: u32 = 0x1304;
    const PED_INFO: u32 = 0x21C;
    const PED_INFO_TYPE: u32 = 0x12C;
    const MODEL_TABLE_VA: u32 = 0x0129_5CD8;
    const SPECIAL_MODEL_STATUS: u32 = 0x57;
    const PED_TYPE: u32 = 2;
    const CLEAR_FLAG: u32 = 0x0002_0000;

    let read_u32 = |address: u32| unsafe { core::ptr::read(address as *const u32) };
    let read_i16 = |address: u32| unsafe { core::ptr::read(address as *const i16) };
    let read_u8 = |address: u32| unsafe { core::ptr::read(address as *const u8) };
    let write_u32 = |address: u32, value: u32| unsafe { core::ptr::write(address as *mut u32, value) };
    let write_u8 = |address: u32, value: u8| unsafe { core::ptr::write(address as *mut u8, value) };

    let mut vehicle = read_u32(task.wrapping_add(TASK_CAR));
    if vehicle == 0 || read_u8(task.wrapping_add(TASK_MODE)) != 0 {
        return 1;
    }

    if read_u32(task.wrapping_add(TASK_SUBTASK)) == 0 {
        let wheel = lf_checker_rt::callee_thiscall!(
            1, u32, vehicle, read_u32(task.wrapping_add(TASK_PED))
        );
        write_u32(task.wrapping_add(TASK_STATE), 0);
        if wheel != 0 {
            write_u32(task.wrapping_add(TASK_STATE), read_u32(wheel.wrapping_add(0x1C)));
        }
        let _ = lf_checker_rt::callee_thiscall!(2, u32, task, ped);
    }

    vehicle = read_u32(task.wrapping_add(TASK_CAR));
    let vehicle_state = read_u32(vehicle.wrapping_add(CAR_STATE));
    if vehicle_state != 0
        && read_u8(vehicle_state.wrapping_add(CAR_STATE_READY)) != 0
        && read_u8(task.wrapping_add(TASK_FLAG)) != 0
        && ped != 0
    {
        let ped_info = read_u32(ped.wrapping_add(PED_INFO));
        if read_u32(ped_info.wrapping_add(PED_INFO_TYPE)) == PED_TYPE {
            let flags = read_u32(vehicle_state.wrapping_add(CAR_STATE_FLAGS));
            write_u32(vehicle_state.wrapping_add(CAR_STATE_FLAGS), flags & !CLEAR_FLAG);
            write_u8(task.wrapping_add(TASK_COMPLETE), 1);
        }
    }

    vehicle = read_u32(task.wrapping_add(TASK_CAR));
    let model_index = read_i16(vehicle.wrapping_add(CAR_MODEL_INDEX)) as i32;
    let table_slot = MODEL_TABLE_VA.wrapping_add(model_index.wrapping_mul(4) as u32);
    let model = unsafe { core::ptr::read(lf_checker_rt::global::<u32>(table_slot)) };
    let subtask = read_u32(task.wrapping_add(TASK_SUBTASK));

    if read_u32(model.wrapping_add(CAR_MODEL_STATUS)) == SPECIAL_MODEL_STATUS {
        let _ = lf_checker_rt::callee_thiscall!(3, u32, vehicle, ped, 5, subtask, 0);
    } else if read_u32(vehicle.wrapping_add(CAR_FALLBACK_STATUS)) == 1 {
        let _ = lf_checker_rt::callee_thiscall!(4, u32, vehicle, ped, 5);
    } else {
        let _ = lf_checker_rt::callee_thiscall!(
            5, u32, vehicle, ped, read_u32(task.wrapping_add(TASK_PED))
        );
    }
    0
});
