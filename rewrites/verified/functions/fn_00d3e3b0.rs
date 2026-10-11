// original: 0x00D3E3B0 task_registry_register_entry (proposed)
/// Removes a previous registration when the object's signed 16-bit task ID
/// is neither -1 nor the requested 32-bit ID. If the registered flag is
/// clear, it asks the singleton registry to add or update the requested ID
/// and metric. A true registry result sets bit 0 of the object's flag byte
/// and increments the registry reference count. The object is then assigned
/// the requested ID through a checked thiscall whenever that flag is set;
/// the byte return reports whether assignment occurred.
lf_checker_rt::export!(thiscall, rw_d3e3b0(this: u32, record_id: u32, metric: i32) -> u8 {
    const REMOVE_OLD_ENTRY: u32 = 1;
    const REGISTRY_UPDATE: u32 = 2;
    const REGISTRY_ADD_REFERENCE: u32 = 3;
    const OBJECT_SET_ID: u32 = 4;
    const REGISTRY_OBJECT: u32 = 0x0172_05A0;
    const TASK_ID_OFFSET: usize = 8;
    const FLAGS_OFFSET: usize = 12;
    const REGISTERED: u8 = 1;
    let object = this as *mut u8;
    let old_id = unsafe { (object.add(TASK_ID_OFFSET) as *const u16).read_unaligned() };
    if old_id != u16::MAX && (old_id as i16 as i32 as u32) != record_id {
        let _ = lf_checker_rt::callee_thiscall!(REMOVE_OLD_ENTRY, u8, this);
    }

    let flags = unsafe { object.add(FLAGS_OFFSET).read() };
    if flags & REGISTERED == 0 {
        let registry = lf_checker_rt::relocated(REGISTRY_OBJECT);
        let already_registered = lf_checker_rt::callee_thiscall!(REGISTRY_UPDATE, u8, registry, record_id, metric as u32);
        if already_registered != 0 {
            unsafe { object.add(FLAGS_OFFSET).write(flags | REGISTERED); }
            let _ = lf_checker_rt::callee_thiscall!(REGISTRY_ADD_REFERENCE, u8, registry, record_id, metric as u32);
        }
    }

    if unsafe { object.add(FLAGS_OFFSET).read() } & REGISTERED != 0 {
        lf_checker_rt::callee_thiscall!(OBJECT_SET_ID, u8, this, record_id)
    } else {
        0
    }
});
