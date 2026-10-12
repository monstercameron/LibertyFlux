// original: 0x00D16210 CTaskComplexCombatBustPed::select_sub_task_id

/// Select the bust-ped subtask from the ped state, two virtual answers, and a
/// target's position. ECX points to the task and two stack words provide the
/// target object and an unused value. A state-byte/status pair can return the
/// fallback id before calls. Otherwise the ped's vtable slot at +0x128 is
/// called; when its AL result is true, the function subtracts the target's
/// three coordinates from the ped's coordinates, squares each difference,
/// adds them in source order, and returns the distance id only when the sum is
/// strictly below the relocated image threshold. An unordered float compare
/// takes the same branch as the original. If execution reaches the second
/// virtual call, its AL result selects one of two ids. Thiscall consumes both
/// stack words; the unused word is still preserved in the signature.
lf_checker_rt::export!(thiscall, rw_D16210(this: u32, target: u32, _unused: u32) -> u32 {
    const PED_POINTER: u32 = 0x18;
    const PED_FLAGS: u32 = 0x26C;
    const PED_STATUS: u32 = 0xB30;
    const VTABLE_SLOT: u32 = 0x128;
    const POSITION_POINTER: u32 = 0x20;
    const POSITION_X: u32 = 0x30;
    const POSITION_Y: u32 = 0x34;
    const POSITION_Z: u32 = 0x38;
    const DISTANCE_LIMIT: u32 = 0x00FE8AB8;
    const FALLBACK_ID: u32 = 0x2DE;
    const DISTANCE_ID: u32 = 0x44C;
    const ANSWER_TRUE_ID: u32 = 0xC8;
    const ANSWER_FALSE_ID: u32 = 0x2E8;

    let ped = unsafe { core::ptr::read_unaligned((this + PED_POINTER) as *const u32) };
    let flags = unsafe { core::ptr::read((ped + PED_FLAGS) as *const u8) };
    let status = unsafe { core::ptr::read_unaligned((ped + PED_STATUS) as *const u32) };
    if flags & 4 != 0 && status != 0 {
        return FALLBACK_ID;
    }

    let virtual_entry = unsafe {
        let vtable = core::ptr::read_unaligned(ped as *const u32);
        core::ptr::read_unaligned((vtable + VTABLE_SLOT) as *const u32)
    };
    let virtual_call: extern "thiscall" fn(u32) -> u8 =
        unsafe { core::mem::transmute(virtual_entry as usize) };
    if virtual_call(ped) != 0 {
        let target_position = unsafe { core::ptr::read_unaligned((target + POSITION_POINTER) as *const u32) };
        let ped_position = unsafe { core::ptr::read_unaligned((ped + POSITION_POINTER) as *const u32) };
        let ped_y = unsafe { core::ptr::read_unaligned((ped_position + POSITION_Y) as *const f32) };
        let target_y = unsafe { core::ptr::read_unaligned((target_position + POSITION_Y) as *const f32) };
        let ped_x = unsafe { core::ptr::read_unaligned((ped_position + POSITION_X) as *const f32) };
        let target_x = unsafe { core::ptr::read_unaligned((target_position + POSITION_X) as *const f32) };
        let ped_z = unsafe { core::ptr::read_unaligned((ped_position + POSITION_Z) as *const f32) };
        let target_z = unsafe { core::ptr::read_unaligned((target_position + POSITION_Z) as *const f32) };

        let dy = core::hint::black_box(ped_y) - core::hint::black_box(target_y);
        let dx = core::hint::black_box(ped_x) - core::hint::black_box(target_x);
        let dz = core::hint::black_box(ped_z) - core::hint::black_box(target_z);
        let dy_squared = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dx_squared = core::hint::black_box(dx) * core::hint::black_box(dx);
        let dz_squared = core::hint::black_box(dz) * core::hint::black_box(dz);
        let xy_squared = core::hint::black_box(dy_squared) + core::hint::black_box(dx_squared);
        let distance_squared = core::hint::black_box(xy_squared) + core::hint::black_box(dz_squared);
        let limit = unsafe {
            core::ptr::read(lf_checker_rt::relocated(DISTANCE_LIMIT) as *const f32)
        };
        if core::hint::black_box(limit) > core::hint::black_box(distance_squared) {
            return DISTANCE_ID;
        }
    }

    let virtual_entry = unsafe {
        let vtable = core::ptr::read_unaligned(ped as *const u32);
        core::ptr::read_unaligned((vtable + VTABLE_SLOT) as *const u32)
    };
    let virtual_call: extern "thiscall" fn(u32) -> u8 =
        unsafe { core::mem::transmute(virtual_entry as usize) };
    if virtual_call(ped) != 0 { ANSWER_TRUE_ID } else { ANSWER_FALSE_ID }
});
