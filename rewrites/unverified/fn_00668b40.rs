// original: 0x00668B40 rage::ptxEventEmitter::vf11

/// Uses the manager pointer stored at global 0x1BB6674. It calls manager slot +0x0c with the key at this +0x40 and stores the returned event object at +0x48. A null or empty C string at +0x44 triggers event slot +0x0c and the two direct string helpers in order, with the second helper conditional on the first helper's AL result. It then calls manager slot +4 with the string pointer, stores that result at +0x4c, and calls slot +4 on each non-null result object. The function returns AL=1. All direct helpers and vtable calls are scripted by the contract.
lf_checker_rt::export!(thiscall, rw_00668B40(this: u32) -> u32 {
    const MANAGER_GLOBAL_VA: u32 = 0x01BB6674;
    const LOOKUP_SLOT: u32 = 0x0c;
    const FIND_SLOT: u32 = 0x04;
    const STRING_FIELD: u32 = 0x44;
    const EVENT_FIELD: u32 = 0x48;
    const RESULT_FIELD: u32 = 0x4c;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned() };
    let manager_vtable = unsafe { read_u32(manager) };
    let lookup_target = unsafe { read_u32(manager_vtable.wrapping_add(LOOKUP_SLOT)) };
    let lookup: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(lookup_target as usize) };
    let event_object = lookup(manager, unsafe { read_u32(this.wrapping_add(0x40)) });
    unsafe { write_u32(this.wrapping_add(EVENT_FIELD), event_object) };
    let string_pointer = unsafe { read_u32(this.wrapping_add(STRING_FIELD)) };
    let string_is_empty = if string_pointer == 0 {
        true
    } else {
        let bytes = string_pointer as *const u8;
        unsafe { bytes.read() == 0 }
    };
    if string_is_empty {
        let event_vtable = unsafe { read_u32(event_object) };
        let event_lookup_target = unsafe { read_u32(event_vtable.wrapping_add(LOOKUP_SLOT)) };
        let event_lookup: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(event_lookup_target as usize) };
        let helper_value = event_lookup(event_object);
        let string_field = this.wrapping_add(STRING_FIELD);
        let accepted = lf_checker_rt::callee_thiscall!(5, u8, string_field, helper_value);
        if accepted == 0 {
            let _ignored_result = lf_checker_rt::callee_thiscall!(6, u32, string_field, helper_value);
        }
    }
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned() };
    let manager_vtable = unsafe { read_u32(manager) };
    let find_target = unsafe { read_u32(manager_vtable.wrapping_add(FIND_SLOT)) };
    let find: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(find_target as usize) };
    let result_object = find(manager, string_pointer);
    unsafe { write_u32(this.wrapping_add(RESULT_FIELD), result_object) };
    let stored_event = unsafe { read_u32(this.wrapping_add(EVENT_FIELD)) };
    if stored_event != 0 {
        let vtable = unsafe { read_u32(stored_event) };
        let target = unsafe { read_u32(vtable.wrapping_add(FIND_SLOT)) };
        let method: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
        let _ignored_result = method(stored_event);
    }
    if result_object != 0 {
        let vtable = unsafe { read_u32(result_object) };
        let target = unsafe { read_u32(vtable.wrapping_add(FIND_SLOT)) };
        let method: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
        let _ignored_result = method(result_object);
    }
    1
});
