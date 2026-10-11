// original: 0x00b4ba30 CEventStuckInAir::vf20

/// Compare two event predicates through their vtable methods. The event
/// in the stack argument is queried for its signed kind at slot +8; kinds
/// below 0x68 then use slot +4 and return one unless that answer is 0x5D.
/// Otherwise, and for the 0x5D case, the +8 answers from the argument and
/// this object are compared as signed 32-bit values. `setge`/`(an instruction of the original)`
/// changes only AL, so EAX's upper bytes retain the last method result.
/// Both objects use a vtable pointer at +0; the argument is caller-cleaned
/// by the thiscall entry and its single stack word is popped on return.
lf_checker_rt::export!(thiscall, rw_00B4BA30(this_event: u32, other_event: u32) -> u32 {
    const TYPE_SLOT: u32 = 8;
    const PRIORITY_SLOT: u32 = 4;
    const SECONDARY_THRESHOLD: i32 = 0x68;
    const REJECTED_PRIORITY: u32 = 0x5D;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    #[inline(always)]
    unsafe fn call_method(object: u32, vtable_offset: u32) -> u32 {
        const VTABLE: u32 = 0;
        let vtable = unsafe { (object.wrapping_add(VTABLE) as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(vtable_offset) as *const u32).read_unaligned() };
        let method: Method = unsafe { core::mem::transmute(target as usize) };
        unsafe { method(object) }
    }

    // SAFETY: contracts plant a callable recorder stub at each requested slot.
    let first_type = unsafe { call_method(other_event, TYPE_SLOT) };
    if (first_type as i32) < SECONDARY_THRESHOLD {
        // SAFETY: this slot is planted in the same fabricated vtable.
        let priority = unsafe { call_method(other_event, PRIORITY_SLOT) };
        if priority != REJECTED_PRIORITY {
            return (priority & !0xFF) | 1;
        }
    }

    // SAFETY: each call goes through the event's scripted +8 method slot.
    let other_type = unsafe { call_method(other_event, TYPE_SLOT) };
    let this_type = unsafe { call_method(this_event, TYPE_SLOT) };
    (this_type & !0xFF) | u32::from((this_type as i32) >= (other_type as i32))
});
