// original: 0x00a2b9f0 player_ped_update_task_flags

/// Visit seven task indices in the manager's table. For each scripted task,
/// skip null tasks and status byte `+0xa60 == 2`; otherwise call the ped's
/// virtual slot at `+0x30`, set task flag bit 0 unless byte `+0x26c` has bit 2,
/// and toggle task state bit 12 when it differs from the corresponding global
/// byte. The method returns the last value left in EAX by that loop iteration.
lf_checker_rt::export!(thiscall, rw_a2b9f0(this: u32) -> u32 {
    const MANAGER_FIELD: u32 = 0x0228;
    const MANAGER_SUBOBJECT: u32 = 0x0070;
    const TASK_INDEX: u32 = 0x03d4;
    const TASK_TABLE: u32 = 0x01666c90;
    const TASK_STRIDE: u32 = 0x5c;
    const BYTE_TABLE: u32 = 0x12dd630;
    let manager_ptr = unsafe { ((this + MANAGER_FIELD) as *const u32).read_unaligned() };
    let manager = manager_ptr.wrapping_add(MANAGER_SUBOBJECT);
    let table_index = unsafe { ((manager + TASK_INDEX) as *const u32).read_unaligned() };
    let data_this = lf_checker_rt::relocated(TASK_TABLE).wrapping_add(table_index.wrapping_mul(TASK_STRIDE));
    let mut eax = 0u32;
    for task_index in 0..7u32 {
        let task = unsafe { lf_checker_rt::callee_thiscall!(1, u32, data_this, task_index) };
        eax = task;
        if task == 0 { continue; }
        let status = unsafe { ((task + 0x0a60) as *const u8).read_unaligned() };
        if status == 2 { continue; }
        let _ignored = unsafe { lf_checker_rt::callee_thiscall!(2, u32, this, 1u32) };
        let task_flags = unsafe { ((task + 0x026c) as *const u8).read_unaligned() };
        if task_flags & 4 == 0 {
            let word = unsafe { ((task + 0x0024) as *const u32).read_unaligned() };
            unsafe { ((task + 0x0024) as *mut u32).write_unaligned(word | 1) };
        }
        let table_byte = unsafe { lf_checker_rt::global::<u8>(BYTE_TABLE.wrapping_add(task_index)).read_unaligned() } as u32;
        let task_state = unsafe { ((task + 0x0260) as *const u32).read_unaligned() };
        eax = (table_byte.wrapping_shl(12) ^ task_state) & 0x1000;
        unsafe { ((task + 0x0260) as *mut u32).write_unaligned(task_state ^ eax) };
    }
    eax
});
