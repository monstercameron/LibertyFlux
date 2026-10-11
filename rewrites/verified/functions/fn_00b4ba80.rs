// original: 0x00b4ba80 CEventVehicleCollision::vf20

/// Return the event's `+4` vtable method result with AL replaced by whether
/// that result differs from 0x79. The method argument arrives on the stack;
/// the incoming thiscall ECX value is unused by the original. The method's
/// high EAX bytes are preserved, matching the byte-sized boolean write.
lf_checker_rt::export!(thiscall, rw_00B4BA80(_this_event: u32, event: u32) -> u32 {
    const VTABLE: u32 = 0;
    const METHOD_SLOT: u32 = 4;
    const REJECTED_KIND: u32 = 0x79;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    // SAFETY: the contract plants a callable recorder stub at vtable +4.
    let vtable = unsafe { (event.wrapping_add(VTABLE) as *const u32).read_unaligned() };
    let target = unsafe { (vtable.wrapping_add(METHOD_SLOT) as *const u32).read_unaligned() };
    let method: Method = unsafe { core::mem::transmute(target as usize) };
    let answer = unsafe { method(event) };
    (answer & !0xFF) | u32::from(answer != REJECTED_KIND)
});
