// original: 0x00D39B60 ped_task_lookup_and_store_registry_task

/// When the current task object at +0x10 is null, check that task lookup is enabled and that the 16-bit task key at +0x36 is nonzero. Query the registry, invoke its method at vtable slot 1, and store the method result at +0x10. This function has no return value.
lf_checker_rt::export!(thiscall, rw_00d39b60(this: u32) -> u32 {
    unsafe {
        const TASK_OBJECT: u32 = 0x10;
        const TASK_KEY: u32 = 0x36;
        const REGISTRY: u32 = 1;
        const LOOKUP: u32 = 2;
        const REGISTRY_METHOD: u32 = 3;
        if (this.wrapping_add(TASK_OBJECT) as *const u32).read_unaligned() == 0
            && lf_checker_rt::callee_cdecl!(REGISTRY, u32,) as u8 != 0
        {
            let key = (this.wrapping_add(TASK_KEY) as *const u16).read_unaligned() as u32;
            if key != 0 {
                let registry = lf_checker_rt::relocated(0x01908ef0);
                let object = lf_checker_rt::callee_thiscall!(LOOKUP, u32, registry, key, 0);
                if object != 0 {
                    let task = lf_checker_rt::callee_thiscall!(REGISTRY_METHOD, u32, object);
                    (this.wrapping_add(TASK_OBJECT) as *mut u32).write_unaligned(task);
                }
            }
        }
        0
    }
});
