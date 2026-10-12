// original: 0x00b4b8b0 CEventLeaderEnteredCarAsDriver::vf20

/// Query the argument event's +4 method and return its full EAX value with
/// the low byte replaced by whether that value differs from 0x42. The
/// original is a one-argument stdcall helper; the vtable method is thiscall.
lf_checker_rt::export!(stdcall, rw_00b4b8b0(other_event: u32) -> u32 {
    const METHOD_SLOT: u32 = 4;
    const DRIVER_ENTRY_TYPE: u32 = 0x42;
    type Method = unsafe extern "thiscall" fn(u32) -> u32;

    let vtable = unsafe { (other_event as *const u32).read_unaligned() };
    let target = unsafe { (vtable.wrapping_add(METHOD_SLOT) as *const u32).read_unaligned() };
    let method: Method = unsafe { core::mem::transmute(target as usize) };
    // SAFETY: the contract plants a recorder stub in the argument's +4 slot.
    let event_type = unsafe { method(other_event) };
    (event_type & !0xFF) | u32::from(event_type != DRIVER_ENTRY_TYPE)
});
