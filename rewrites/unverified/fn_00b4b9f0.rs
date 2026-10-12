// original: 0x00b4b9f0 CEventShocking::vf20

/// Compare event types through vtable slots +4 and +8. If the argument's
/// +4 method returns exactly 0x79, return that word with AL cleared. For any
/// other type, compare the +8 answers as signed 32-bit values and replace
/// only AL with the greater-or-equal result.
lf_checker_rt::export!(thiscall, rw_00b4b9f0(this_event: u32, other_event: u32) -> u32 {
    const TYPE_SLOT: u32 = 4;
    const PRIORITY_SLOT: u32 = 8;
    const EXCLUDED_TYPE: u32 = 0x79;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_event_method(object: u32, slot: u32) -> u32 {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(slot) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    // SAFETY: the contract plants a recorder stub in each requested slot.
    let other_type = unsafe { call_event_method(other_event, TYPE_SLOT) };
    if other_type == EXCLUDED_TYPE {
        return other_type & !0xFF;
    }
    let other_priority = unsafe { call_event_method(other_event, PRIORITY_SLOT) };
    let this_priority = unsafe { call_event_method(this_event, PRIORITY_SLOT) };
    (this_priority & !0xFF) | u32::from((this_priority as i32) >= (other_priority as i32))
});
