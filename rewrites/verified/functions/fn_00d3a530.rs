// original: 0x00D3A530 CDummyTask_Dead::vf9

/// Check whether the task system is active and, if so, ask whether the ped is already in the requested state. When the ped is present, its task state at +0x7b8 is not 1, and the helper did not report an active state, clear bits 0 and 1 at data_object+0xf4, set bit 1 in its nested byte object, and mark this task at +0x20. The function has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00d3a530(this: u32, data_object: u32, ped: u32) -> u32 {
    unsafe {
        const SYSTEM_ACTIVE: u32 = 1;
        const PED_QUERY: u32 = 2;
        const PED_TASK_STATE: u32 = 0x7b8;
        const DATA_FLAGS: u32 = 0xf4;
        const NESTED_BYTE_POINTER: u32 = 0x78;
        const TASK_MARKER: u32 = 0x20;
        let system_ready = lf_checker_rt::callee_cdecl!(SYSTEM_ACTIVE, u32,) as u8 != 0;
        let already_active = if system_ready {
            lf_checker_rt::callee_thiscall!(PED_QUERY, u32, ped, 0) as u8 == 0
        } else {
            false
        };
        if ped != 0
            && (ped.wrapping_add(PED_TASK_STATE) as *const u32).read_unaligned() != 1
            && !already_active
        {
            let flags_address = data_object.wrapping_add(DATA_FLAGS);
            let flags = (flags_address as *const u8).read_unaligned();
            (flags_address as *mut u8).write_unaligned(flags & !0x03);
            let nested = (data_object.wrapping_add(NESTED_BYTE_POINTER) as *const u32).read_unaligned();
            (nested as *mut u8).write_unaligned((nested as *const u8).read_unaligned() | 0x02);
            (this.wrapping_add(TASK_MARKER) as *mut u8).write_unaligned(1);
        }
        0
    }
});
