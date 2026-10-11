// original: 0x00D39BA0 CDummyTask_Dead::vf2

/// When the current task object is absent and the 16-bit task key at +0x3c is nonzero, check that task lookup is enabled, query the registry, invoke its method at vtable slot 1, and pass the result to the reference-update helper. The helper call may update this object's +0x10 field; the function itself has no return value.
lf_checker_rt::export!(thiscall, rw_00d39ba0(this: u32) -> u32 {
    unsafe {
        const TASK_OBJECT: u32 = 0x10;
        const TASK_KEY: u32 = 0x3c;
        const REGISTRY: u32 = 1;
        const LOOKUP: u32 = 2;
        const REGISTRY_METHOD: u32 = 3;
        const STORE_TASK_REFERENCE: u32 = 4;
        if (this.wrapping_add(TASK_OBJECT) as *const u32).read_unaligned() == 0 {
            let key = (this.wrapping_add(TASK_KEY) as *const u16).read_unaligned() as u32;
            if key != 0 && lf_checker_rt::callee_cdecl!(REGISTRY, u32,) as u8 != 0 {
                let registry = lf_checker_rt::relocated(0x01908ef0);
                let object = lf_checker_rt::callee_thiscall!(LOOKUP, u32, registry, 0, key);
                if object != 0 {
                    let task = lf_checker_rt::callee_thiscall!(REGISTRY_METHOD, u32, object);
                    let _ = lf_checker_rt::callee_thiscall!(STORE_TASK_REFERENCE, u32, this, task);
                }
            }
        }
        0
    }
});
