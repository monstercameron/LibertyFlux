// original: 0x00B44F70 CEventAttractor::CEventAttractor
/// Construct an attractor event with a referenced entity, a setting word and
/// one byte of flags. The base constructor runs first; a nonzero setting word
/// is forwarded by address to the reference helper. The returned value is the
/// caller-provided object pointer.
lf_checker_rt::export!(thiscall, rw_00b44f70(
    this: u32, referenced_entity: u32, setting: u32, flags: u32
) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0x00eade84;
        const ENTITY_FIELD: usize = 0x18;
        const SETTING_FIELD: usize = 0x1c;
        const FLAGS_FIELD: usize = 0x20;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let object = this as *mut u8;
        let entity_field = object.add(ENTITY_FIELD);
        (entity_field as *mut u32).write_unaligned(referenced_entity);
        (object.add(SETTING_FIELD) as *mut u32).write_unaligned(setting);
        object.add(FLAGS_FIELD).write(flags as u8);
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        if setting != 0 {
            let _ = lf_checker_rt::callee_stdcall!(2, u32, object.add(SETTING_FIELD) as u32);
        }
        this
    }
});
