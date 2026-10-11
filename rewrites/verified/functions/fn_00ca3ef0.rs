// original: 0x00CA3EF0 CEventDeath::vf4

/// Read the current event context pointer from the context global. If it is null, return zero. Otherwise pass the unsigned event byte at +0x0C and the dword at +0x10 to the event-context handler, then return that call's EAX value. The object pointer arrives in ECX.
lf_checker_rt::export!(thiscall, rw_00ca3ef0(this: u32) -> u32 {
    const CONTEXT_GLOBAL_VA: u32 = 0x016669C8;
    const EVENT_CODE: u32 = 0x0C;
    const EVENT_SOURCE: u32 = 0x10;
    const CONTEXT_LOOKUP: u32 = 1;
    const EVENT_HANDLER: u32 = 2;
    let context_global = unsafe { lf_checker_rt::global::<u32>(CONTEXT_GLOBAL_VA).read_unaligned() };
    let context = lf_checker_rt::callee_thiscall!(CONTEXT_LOOKUP, u32, context_global);
    if context == 0 {
        0
    } else {
        let event_code = unsafe { (this.wrapping_add(EVENT_CODE) as *const u8).read_unaligned() } as u32;
        let event_source = unsafe { (this.wrapping_add(EVENT_SOURCE) as *const u32).read_unaligned() };
        lf_checker_rt::callee_thiscall!(EVENT_HANDLER, u32, context, event_code, event_source)
    }
});
