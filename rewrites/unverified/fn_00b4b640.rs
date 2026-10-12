// original: 0x00b4b640 CEventAcquaintancePedHate::vf20

/// Compare event kinds first. Equal kinds compare the associated entity's
/// hate byte at +0x219: this event is accepted only when its entity is live,
/// its byte is nonzero, and the other entity is null or has a zero byte.
/// Different kinds fall back to a signed greater-or-equal comparison of
/// the two +8 method results. Return paths preserve the high EAX bytes left
/// by the final pointer or method value, changing only AL.
lf_checker_rt::export!(thiscall, rw_00b4b640(this_event: u32, other_event: u32) -> u32 {
    const TYPE_SLOT: u32 = 4;
    const PRIORITY_SLOT: u32 = 8;
    const ENTITY_POINTER: u32 = 0x18;
    const HATE_FLAG: u32 = 0x219;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_event_method(object: u32, slot: u32) -> u32 {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(slot) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    // SAFETY: the contract replaces both event objects' requested slots.
    let other_kind = unsafe { call_event_method(other_event, TYPE_SLOT) };
    let this_kind = unsafe { call_event_method(this_event, TYPE_SLOT) };
    if other_kind == this_kind {
        let this_entity = unsafe { read_u32(this_event.wrapping_add(ENTITY_POINTER)) };
        if this_entity == 0 {
            return 0;
        }
        let this_hates = unsafe { read_u8(this_entity.wrapping_add(HATE_FLAG)) } != 0;
        let other_entity = unsafe { read_u32(other_event.wrapping_add(ENTITY_POINTER)) };
        let other_hates = other_entity != 0
            && unsafe { read_u8(other_entity.wrapping_add(HATE_FLAG)) } != 0;
        let accepted = this_hates && (!other_hates);
        return (this_entity & !0xFF) | u32::from(accepted);
    }
    let other_priority = unsafe { call_event_method(other_event, PRIORITY_SLOT) };
    let this_priority = unsafe { call_event_method(this_event, PRIORITY_SLOT) };
    (this_priority & !0xFF) | u32::from((this_priority as i32) >= (other_priority as i32))
});
