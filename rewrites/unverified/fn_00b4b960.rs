// original: 0x00b4b960 CEventOnFire::vf20

/// Return true when the argument event's first signed +8 answer is at least
/// 0x5f and its next answer is at most 0x60. Otherwise compare the argument
/// and this object's next +8 answers as signed 32-bit values. Return values
/// change only AL, preserving the high three bytes from the last method call.
lf_checker_rt::export!(thiscall, rw_00b4b960(this_event: u32, other_event: u32) -> u32 {
    const TYPE_SLOT: u32 = 8;
    const LOWER_BOUND: i32 = 0x5F;
    const UPPER_BOUND: i32 = 0x60;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_event_method(object: u32) -> u32 {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(TYPE_SLOT) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    // SAFETY: the contract plants a recorder stub in each object's +8 slot.
    let first_answer = unsafe { call_event_method(other_event) };
    if (first_answer as i32) >= LOWER_BOUND {
        let second_answer = unsafe { call_event_method(other_event) };
        if (second_answer as i32) <= UPPER_BOUND {
            return (second_answer & !0xFF) | 1;
        }
    }
    let other_value = unsafe { call_event_method(other_event) };
    let this_value = unsafe { call_event_method(this_event) };
    (this_value & !0xFF) | u32::from((this_value as i32) >= (other_value as i32))
});
