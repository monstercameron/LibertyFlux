// original: 0x00D3E2F0 task_registry_remove_entry (proposed)
/// Removes this object's registered task identifier when its registered flag
/// is set, clears that flag, and always notifies the object of identifier -1.
/// The identifier is a signed 16-bit field at offset 8; the flag is bit 0 of
/// the byte at offset 12. Registry and object notifications are direct
/// thiscall calls whose arguments are checked by the contract.
lf_checker_rt::export!(thiscall, rw_d3e2f0(this: u32) -> () {
    const REGISTRY_REMOVE: u32 = 1;
    const OBJECT_SET_ID: u32 = 2;
    const REGISTRY_OBJECT: u32 = 0x0172_05A0;
    const TASK_ID_OFFSET: usize = 8;
    const FLAGS_OFFSET: usize = 12;
    const REGISTERED: u8 = 1;

    unsafe {
        let object = this as *mut u8;
        let flags = object.add(FLAGS_OFFSET);
        if flags.read() & REGISTERED != 0 {
            let task_id = (object.add(TASK_ID_OFFSET) as *const i16).read_unaligned();
            let registry = lf_checker_rt::relocated(REGISTRY_OBJECT);
            let _ = lf_checker_rt::callee_thiscall!(REGISTRY_REMOVE, u8, registry, task_id as i32 as u32);
        }
        flags.write(flags.read() & !REGISTERED);
    }
    let _ = lf_checker_rt::callee_thiscall!(OBJECT_SET_ID, u8, this, u32::MAX);
});
