// original: 0x00b4b6c0 CEventBuildingCollision::vf20

/// Compare the results of the +4 event methods on the argument object and
/// this object. The method returns the second call's upper EAX bytes and a
/// low byte indicating whether the first and second 32-bit answers differ.
/// Both calls use the thiscall convention with only the object in ECX.
lf_checker_rt::export!(thiscall, rw_00b4b6c0(this_event: u32, other_event: u32) -> u32 {
    const METHOD_SLOT: u32 = 4;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_event_method(object: u32) -> u32 {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(METHOD_SLOT) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    // SAFETY: the contract plants a recorder stub in each object's +4 slot.
    let other_answer = unsafe { call_event_method(other_event) };
    let this_answer = unsafe { call_event_method(this_event) };
    (this_answer & !0xFF) | u32::from(other_answer != this_answer)
});
