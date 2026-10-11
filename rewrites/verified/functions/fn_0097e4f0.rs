// original: 0x0097E4F0 ped_task_audio_dispatch_two_fields

/// For a ped, finds the nested task entry and forwards its two
/// unaligned values to the embedded audio entity.
///
/// The first cdecl input word is unused; the second is a ped pointer.
/// It reads the pointer at `+0x2c4`, then `+0x25c`, then `+0x10`.
/// If any link is null it returns without a call. Otherwise it calls
/// the same thiscall helper twice with ECX = ped + `0x3c0`, passing
/// the unaligned dwords at entry `+0x26` and `+0x2a`. The function
/// writes no memory and has no semantic return value.
lf_checker_rt::export!(cdecl, rw_0097E4F0(_unused: u32, ped: u32) -> u32 {
    const PED_TASKS: u32 = 0x2C4;
    const TASK_TABLE: u32 = 0x25C;
    const TASK_ENTRY: u32 = 0x10;
    const PED_AUDIO_ENTITY: u32 = 0x3C0;
    const AUDIO_HELPER_ID: u32 = 1;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    let task_table = unsafe { read_u32(ped.wrapping_add(PED_TASKS)) };
    if task_table == 0 { return 0; }
    let task_record = unsafe { read_u32(task_table.wrapping_add(TASK_TABLE)) };
    if task_record == 0 { return 0; }
    let task_values = unsafe { read_u32(task_record.wrapping_add(TASK_ENTRY)) };
    if task_values == 0 { return 0; }
    let first_value = unsafe { read_u32(task_values.wrapping_add(0x26)) };
    let second_value = unsafe { read_u32(task_values.wrapping_add(0x2A)) };
    let audio_entity = ped.wrapping_add(PED_AUDIO_ENTITY);
    let _ = lf_checker_rt::callee_thiscall!(AUDIO_HELPER_ID, u32, audio_entity, first_value);
    let _ = lf_checker_rt::callee_thiscall!(AUDIO_HELPER_ID, u32, audio_entity, second_value);
    0
});
