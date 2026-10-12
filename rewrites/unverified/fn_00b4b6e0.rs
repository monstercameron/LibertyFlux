// original: 0x00b4b6e0 CEventScriptCommand::vf20

/// Accept the argument event immediately when its +4 method answers 10
/// while this event's byte at +0x14 is nonzero. A second +4 answer of 9
/// also accepts it; otherwise one more +4 answer of 9 is required before
/// comparing the +8 answers as signed 32-bit values. Returns preserve the
/// high three EAX bytes from the last method call and replace only AL.
lf_checker_rt::export!(thiscall, rw_00b4b6e0(this_event: u32, other_event: u32) -> u32 {
    const SPECIAL_FLAG: u32 = 0x14;
    const TYPE_SLOT: u32 = 4;
    const PRIORITY_SLOT: u32 = 8;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_event_method(object: u32, slot: u32) -> u32 {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(slot) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    let special_flag = unsafe { read_u8(this_event.wrapping_add(SPECIAL_FLAG)) } != 0;
    if special_flag {
        let first_type = unsafe { call_event_method(other_event, TYPE_SLOT) };
        if first_type == 10 {
            return (first_type & !0xFF) | 1;
        }
        let second_type = unsafe { call_event_method(other_event, TYPE_SLOT) };
        if second_type == 9 {
            return (second_type & !0xFF) | 1;
        }
    }

    let final_type = unsafe { call_event_method(other_event, TYPE_SLOT) };
    if final_type == 9 {
        return (final_type & !0xFF) | 1;
    }
    let other_value = unsafe { call_event_method(other_event, PRIORITY_SLOT) };
    let this_value = unsafe { call_event_method(this_event, PRIORITY_SLOT) };
    (this_value & !0xFF) | u32::from((this_value as i32) >= (other_value as i32))
});
