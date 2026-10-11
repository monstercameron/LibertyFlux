// original: 0x00CA3F20 CEventDamage::vf21

/// Read the current event context pointer from the context global. If it is null, return zero; otherwise pass the receiver to the damage-event handler and return that call's EAX value. The receiver pointer arrives in ECX.
lf_checker_rt::export!(thiscall, rw_00ca3f20(this: u32) -> u32 {
    const CONTEXT_GLOBAL_VA: u32 = 0x016669C8;
    const CONTEXT_LOOKUP: u32 = 1;
    const DAMAGE_HANDLER: u32 = 2;
    let global_context = unsafe { lf_checker_rt::global::<u32>(CONTEXT_GLOBAL_VA).read_unaligned() };
    let context = lf_checker_rt::callee_thiscall!(CONTEXT_LOOKUP, u32, global_context);
    if context == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(DAMAGE_HANDLER, u32, context, this)
    }
});
