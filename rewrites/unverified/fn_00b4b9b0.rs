// original: 0x00b4b9b0 CEventPedCollisionWithPlayer::vf20

/// First call the argument event's +0x4c method. A nonzero low byte returns
/// that method's upper EAX bytes with AL set to one. Otherwise compare the
/// argument and this object's +8 method answers as signed 32-bit values and
/// replace AL with the greater-or-equal result.
lf_checker_rt::export!(thiscall, rw_00b4b9b0(this_event: u32, other_event: u32) -> u32 {
    const TYPE_SLOT: u32 = 8;
    const PED_QUERY_SLOT: u32 = 0x4C;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_event_method(object: u32, slot: u32) -> u32 {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(slot) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    // SAFETY: both entries are recorder stubs planted by the contract.
    let ped_query = unsafe { call_event_method(other_event, PED_QUERY_SLOT) };
    if ped_query as u8 != 0 {
        return (ped_query & !0xFF) | 1;
    }
    let other_value = unsafe { call_event_method(other_event, TYPE_SLOT) };
    let this_value = unsafe { call_event_method(this_event, TYPE_SLOT) };
    (this_value & !0xFF) | u32::from((this_value as i32) >= (other_value as i32))
});
